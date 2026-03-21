/*
 * Copyright (c) Microsoft Corporation.
 * Licensed under the MIT license.
 */

#include <rte_config.h>
#include <rte_errno.h>
#include <rte_ethdev.h>
#include <rte_ether.h>
#include <rte_mbuf.h>
#include <rte_cycles.h>
#include <rte_lcore.h>
#include <rte_launch.h>

void rte_pktmbuf_free_(struct rte_mbuf *packet)
{
    rte_pktmbuf_free(packet);
}

struct rte_mbuf *rte_pktmbuf_alloc_(struct rte_mempool *mp)
{
    return rte_pktmbuf_alloc(mp);
}

uint16_t rte_eth_tx_burst_(uint16_t port_id, uint16_t queue_id, struct rte_mbuf **tx_pkts, uint16_t nb_pkts)
{
    return rte_eth_tx_burst(port_id, queue_id, tx_pkts, nb_pkts);
}

uint16_t rte_eth_rx_burst_(uint16_t port_id, uint16_t queue_id, struct rte_mbuf **rx_pkts, const uint16_t nb_pkts)
{
    return rte_eth_rx_burst(port_id, queue_id, rx_pkts, nb_pkts);
}

uint16_t rte_mbuf_refcnt_read_(const struct rte_mbuf *m)
{
    return rte_mbuf_refcnt_read(m);
}

uint16_t rte_mbuf_refcnt_update_(struct rte_mbuf *m, int16_t value)
{
    return rte_mbuf_refcnt_update(m, value);
}

char *rte_pktmbuf_adj_(struct rte_mbuf *m, uint16_t len)
{
    return rte_pktmbuf_adj(m, len);
}

int rte_pktmbuf_trim_(struct rte_mbuf *m, uint16_t len)
{
    return rte_pktmbuf_trim(m, len);
}

uint16_t rte_pktmbuf_headroom_(const struct rte_mbuf *m)
{
    return rte_pktmbuf_headroom(m);
}

uint16_t rte_pktmbuf_tailroom_(const struct rte_mbuf *m)
{
    return rte_pktmbuf_tailroom(m);
}

int rte_errno_()
{
    return rte_errno;
}

int rte_pktmbuf_chain_(struct rte_mbuf *head, struct rte_mbuf *tail)
{
    return rte_pktmbuf_chain(head, tail);
}

int rte_eth_rss_ip_()
{
    return RTE_ETH_RSS_IP;
}

int rte_eth_tx_offload_tcp_cksum_()
{
    return RTE_ETH_TX_OFFLOAD_TCP_CKSUM;
}

int rte_eth_rx_offload_tcp_cksum_()
{
    return RTE_ETH_RX_OFFLOAD_TCP_CKSUM;
}

int rte_eth_tx_offload_udp_cksum_()
{
    return RTE_ETH_TX_OFFLOAD_UDP_CKSUM;
}

int rte_eth_rx_offload_udp_cksum_()
{
    return RTE_ETH_RX_OFFLOAD_TCP_CKSUM;
}

int rte_eth_tx_offload_multi_segs_()
{
    return RTE_ETH_TX_OFFLOAD_MULTI_SEGS;
}

char *rte_pktmbuf_prepend_(struct rte_mbuf *m, uint16_t len)
{
    return rte_pktmbuf_prepend(m, len);
}

struct rte_mbuf *rte_mbuf_from_indirect_(struct rte_mbuf *mi)
{
    return rte_mbuf_from_indirect(mi);
}

void rte_pktmbuf_detach_(struct rte_mbuf *m)
{
    rte_pktmbuf_detach(m);
}

uint64_t rte_mbuf_f_tx_ipv4_(void) { return RTE_MBUF_F_TX_IPV4; }
uint64_t rte_mbuf_f_tx_ip_cksum_(void) { return RTE_MBUF_F_TX_IP_CKSUM; }
uint64_t rte_mbuf_f_tx_tcp_seg_(void) { return RTE_MBUF_F_TX_TCP_SEG; }
uint64_t rte_mbuf_f_tx_tcp_cksum_(void) { return RTE_MBUF_F_TX_TCP_CKSUM; }
uint64_t rte_mbuf_f_tx_udp_cksum_(void) { return RTE_MBUF_F_TX_UDP_CKSUM; }
uint64_t rte_eth_tx_offload_tcp_tso_(void) { return RTE_ETH_TX_OFFLOAD_TCP_TSO; }
uint64_t rte_get_tsc_hz_(void) { return rte_get_tsc_hz(); }
uint64_t rte_get_tsc_cycles_(void) { return rte_get_tsc_cycles(); }

/* * Запуск функции на удаленном ядре.
 * f - указатель на функцию типа int (*)(void *)
 */
int rte_eal_remote_launch_(lcore_function_t *f, void *arg, unsigned int slave_id) 
{
    return rte_eal_remote_launch(f, arg, slave_id);
}

/* * Ожидание завершения конкретного ядра.
 * Возвращает 0 при успехе или отрицательное число при ошибке.
 */
int rte_eal_wait_lcore_(unsigned int slave_id) 
{
    return rte_eal_wait_lcore(slave_id);
}

/* * Возвращает состояние ядра (WAIT, RUNNING, FINISHED).
 */
enum rte_lcore_state_t rte_eal_get_lcore_state_(unsigned int lcore_id) 
{
    return rte_eal_get_lcore_state(lcore_id);
}

/* * Поиск следующего доступного ядра.
 * i - текущий ID, skip_main (1 или 0), wrap (зацикливание)
 */
unsigned int rte_get_next_lcore_(unsigned int i, int skip_main, int wrap) 
{
    return rte_get_next_lcore(i, skip_main, wrap);
}

/* * Получение ID текущего ядра (через Thread Local Storage внутри DPDK).
 */
unsigned int rte_lcore_id_(void) 
{
    return rte_lcore_id();
}


