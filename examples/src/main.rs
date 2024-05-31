use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32},
        Arc,
    },
    thread,
    time::Duration,
};

use libcsp_rust::{
    csp_accept_guarded, csp_bind, csp_buffer_get, csp_conn_dport, csp_conn_print_table,
    csp_connect_guarded, csp_iflist_print, csp_init, csp_listen, csp_ping, csp_read, csp_reboot,
    csp_route_work, csp_send, csp_service_handler, ConnectOpts, CspSocket, MsgPriority,
    SocketFlags, CSP_ANY, CSP_LOOPBACK,
};

const MY_SERVER_PORT: i32 = 10;
const RUN_DURATION_IN_SECS: u32 = 3;

const TEST_MODE: bool = false;

/*
#include <csp/csp_debug.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>

#include <csp/csp.h>
#include <csp/drivers/usart.h>
#include <csp/drivers/can_socketcan.h>
#include <csp/interfaces/csp_if_zmqhub.h>


/* These three functions must be provided in arch specific way */
int router_start(void);
int server_start(void);
int client_start(void);

/* Server port, the port the server listens on for incoming connections from the client. */
#define MY_SERVER_PORT		10

/* Commandline options */
static uint8_t server_address = 255;

/* test mode, used for verifying that host & client can exchange packets over the loopback interface */
static bool test_mode = false;
static unsigned int server_received = 0;
static unsigned int run_duration_in_sec = 3;

/* Server task - handles requests from clients */
void server(void) {

    csp_print("Server task started\n");

    /* Create socket with no specific socket options, e.g. accepts CRC32, HMAC, etc. if enabled during compilation */
    csp_socket_t sock = {0};

    /* Bind socket to all ports, e.g. all incoming connections will be handled here */
    csp_bind(&sock, CSP_ANY);

    /* Create a backlog of 10 connections, i.e. up to 10 new connections can be queued */
    csp_listen(&sock, 10);

    /* Wait for connections and then process packets on the connection */
    while (1) {

        /* Wait for a new connection, 10000 mS timeout */
        csp_conn_t *conn;
        if ((conn = csp_accept(&sock, 10000)) == NULL) {
            /* timeout */
            continue;
        }

        /* Read packets on connection, timout is 100 mS */
        csp_packet_t *packet;
        while ((packet = csp_read(conn, 50)) != NULL) {
            switch (csp_conn_dport(conn)) {
            case MY_SERVER_PORT:
                /* Process packet here */
                csp_print("Packet received on MY_SERVER_PORT: %s\n", (char *) packet->data);
                csp_buffer_free(packet);
                ++server_received;
                break;

            default:
                /* Call the default CSP service handler, handle pings, buffer use, etc. */
                csp_service_handler(packet);
                break;
            }
        }

        /* Close current connection */
        csp_close(conn);

    }

    return;

}
/* End of server task */

/* Client task sending requests to server task */
void client(void) {

    csp_print("Client task started\n");

    unsigned int count = 'A';

    while (1) {

        usleep(test_mode ? 200000 : 1000000);

        /* Send ping to server, timeout 1000 mS, ping size 100 bytes */
        int result = csp_ping(server_address, 1000, 100, CSP_O_NONE);
        csp_print("Ping address: %u, result %d [mS]\n", server_address, result);
        (void) result;

        /* Send reboot request to server, the server has no actual implementation of csp_sys_reboot() and fails to reboot */
        csp_reboot(server_address);
        csp_print("reboot system request sent to address: %u\n", server_address);

        /* Send data packet (string) to server */

        /* 1. Connect to host on 'server_address', port MY_SERVER_PORT with regular UDP-like protocol and 1000 ms timeout */
        csp_conn_t * conn = csp_connect(CSP_PRIO_NORM, server_address, MY_SERVER_PORT, 1000, CSP_O_NONE);
        if (conn == NULL) {
            /* Connect failed */
            csp_print("Connection failed\n");
            return;
        }

        /* 2. Get packet buffer for message/data */
        csp_packet_t * packet = csp_buffer_get(100);
        if (packet == NULL) {
            /* Could not get buffer element */
            csp_print("Failed to get CSP buffer\n");
            return;
        }

        /* 3. Copy data to packet */
        memcpy(packet->data, "Hello world ", 12);
        memcpy(packet->data + 12, &count, 1);
        memset(packet->data + 13, 0, 1);
        count++;

        /* 4. Set packet length */
        packet->length = (strlen((char *) packet->data) + 1); /* include the 0 termination */

        /* 5. Send packet */
        csp_send(conn, packet);

        /* 6. Close connection */
        csp_close(conn);
    }

    return;
}
/* End of client task */

static void print_usage(void)
{
    csp_print("Usage:\n"
              " -t               enable test mode\n"
              " -T <duration>    enable test mode with running time in seconds\n"
              " -h               print help\n");
}

/* main - initialization of CSP and start of server/client tasks */
int main(int argc, char * argv[]) {

    uint8_t address = 0;
    int opt;
    while ((opt = getopt(argc, argv, "tT:h")) != -1) {
        switch (opt) {
            case 'a':
                address = atoi(optarg);
                break;
            case 'r':
                server_address = atoi(optarg);
                break;
            case 't':
                test_mode = true;
                break;
            case 'T':
                test_mode = true;
                run_duration_in_sec = atoi(optarg);
                break;
            case 'h':
                print_usage();
                exit(0);
                break;
            default:
                print_usage();
                exit(1);
                break;
        }
    }

    csp_print("Initialising CSP");

    /* Init CSP */
    csp_init();

    /* Start router */
    router_start();

    /* Add interface(s) */
    csp_iface_t * default_iface = NULL;
    if (!default_iface) {
        /* no interfaces configured - run server and client in process, using loopback interface */
        server_address = address;
    }

    csp_print("Connection table\r\n");
    csp_conn_print_table();

    csp_print("Interfaces\r\n");
    csp_iflist_print();

    /* Start server thread */
    server_start();

    /* Start client thread */
    client_start();

    /* Wait for execution to end (ctrl+c) */
    while(1) {
        sleep(run_duration_in_sec);

        if (test_mode) {
            /* Test mode is intended for checking that host & client can exchange packets over loopback */
            if (server_received < 5) {
                csp_print("Server received %u packets\n", server_received);
                exit(1);
            }
            csp_print("Server received %u packets\n", server_received);
            exit(0);
        }
    }

    return 0;
}
*/

fn main() -> Result<(), u32> {
    println!("CSP server example");
    // SAFETY: We only call this once.
    unsafe { csp_init() };

    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_signal_server = stop_signal.clone();
    let stop_signal_client = stop_signal.clone();
    let server_received = Arc::new(AtomicU32::new(0));
    let server_recv_copy = server_received.clone();

    let csp_router_jh = thread::spawn(|| loop {
        if let Err(e) = csp_route_work() {
            match e {
                libcsp_rust::CspError::TimedOut => continue,
                e => {
                    println!("CSP router error: {:?}", e);
                    break;
                }
            }
        }
    });

    let csp_server_jh = thread::spawn(|| {
        server(server_received, stop_signal_server);
    });

    let csp_client_jh = thread::spawn(|| {
        client(stop_signal_client);
    });

    println!("CSP connection table");
    csp_conn_print_table();

    println!("CSP interfaces");
    csp_iflist_print();
    let mut app_result = Ok(());
    // Wait for execution to end (ctrl+c)
    loop {
        std::thread::sleep(Duration::from_secs(RUN_DURATION_IN_SECS as u64));

        if TEST_MODE {
            // Test mode is intended for checking that host & client can exchange packets over loopback
            let received_count = server_recv_copy.load(std::sync::atomic::Ordering::Relaxed);
            println!("CSP: Server received {} packets", received_count);
            if received_count < 5 {
                stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);
                app_result = Err(1);
                break;
            }
            stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);
            break;
        }
    }

    csp_router_jh.join().unwrap();
    csp_server_jh.join().unwrap();
    csp_client_jh.join().unwrap();
    app_result
}

fn server(server_received: Arc<AtomicU32>, stop_signal: Arc<AtomicBool>) {
    println!("server task started");

    // Create socket with no specific socket options, e.g. accepts CRC32, HMAC, etc. if enabled
    // during compilation
    let mut csp_socket = CspSocket::default();

    // Bind socket to all ports, e.g. all incoming connections will be handled here
    csp_bind(&mut csp_socket, CSP_ANY);

    // Create a backlog of 10 connections, i.e. up to 10 new connections can be queued
    csp_listen(&mut csp_socket, 10);

    // Wait for connections and then process packets on the connection
    loop {
        if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        // Wait for a new connection, 10000 mS timeout
        let conn = csp_accept_guarded(&mut csp_socket, Duration::from_millis(10000));
        if conn.is_none() {
            continue;
        }
        let conn = conn.unwrap();

        // Read packets on connection, timout is 100 mS
        loop {
            if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            // SAFETY: Connection is active while we read here.
            let packet = unsafe { csp_read(conn.0, Duration::from_millis(100)) };
            if packet.is_none() {
                break;
            }
            let mut packet = packet.unwrap();
            match csp_conn_dport(conn.0) {
                MY_SERVER_PORT => {
                    server_received.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    // Process packet here.
                    println!(
                        "packet received on MY_SERVER_PORT: {:x?}\n",
                        packet.packet_data()
                    );
                }
                _ => {
                    csp_service_handler(&mut packet);
                }
            };
        }
        // No need to close, we accepted the connection with a guard.
    }
}

fn client(stop_signal: Arc<AtomicBool>) {
    println!("client task started");
    let mut current_letter = 'A';

    loop {
        if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        if TEST_MODE {
            thread::sleep(Duration::from_millis(20));
        } else {
            thread::sleep(Duration::from_millis(100));
        }
        // Send ping to server, timeout 1000 mS, ping size 100 bytes
        if let Err(e) = csp_ping(
            CSP_LOOPBACK,
            Duration::from_millis(1000),
            100,
            SocketFlags::NONE,
        ) {
            println!("ping error: {:?}", e);
        }

        // Send reboot request to server, the server has no actual implementation of
        // csp_sys_reboot() and fails to reboot.
        csp_reboot(CSP_LOOPBACK);
        println!("reboot system request sent to address: {}", CSP_LOOPBACK);

        // Send data packet (string) to server

        // 1. Connect to host on 'server_address', port MY_SERVER_PORT with regular UDP-like
        // protocol and 1000 ms timeout.
        let conn = csp_connect_guarded(
            MsgPriority::Normal,
            CSP_LOOPBACK,
            MY_SERVER_PORT as u8,
            Duration::from_millis(1000),
            ConnectOpts::NONE,
        );
        if conn.is_none() {
            println!("CSP client: connection failed");
            return;
        }
        let conn = conn.unwrap();

        // 2. Get packet buffer for message/data.
        let packet_ref = csp_buffer_get();
        if packet_ref.is_none() {
            println!("CSP client: failed to get CSP buffer");
            return;
        }
        let mut packet_ref = packet_ref.unwrap();

        // 3. Copy data to packet.
        let mut string_to_set = String::from("Hello world");
        string_to_set.push(' ');
        string_to_set.push(current_letter);
        current_letter = (current_letter as u8 + 1) as char;
        string_to_set.push('\0');
        packet_ref.set_data(string_to_set.as_bytes());

        // 4. Send data.
        csp_send(conn.0, packet_ref);
    }
}
