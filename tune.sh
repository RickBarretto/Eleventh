#!/usr/bin/env bash

# Applies OS tuning for high concurrency
# This script also reports which were possible or not to be applied to that system.

report() {
    local description="$1"
    local status="$2"
    printf "%-40s : %s\n" "$description" "$status"
}


systemControl() {
    local key="$1"
    local value="$2"
    if sudo sysctl -w "$key=$value" >/dev/null 2>&1; then
        report "$key=$value" "applied"
    else
        report "$key=$value" "skipped / failed"
    fi
}

ResourceLimitTo() {
    local value="$1"
    if ulimit -n "$value" >/dev/null 2>&1; then
        report "ulimit -n $value" "applied"
    else
        report "ulimit -n $value" "skipped / failed"
    fi
}

systemControl net.core.somaxconn 65535
systemControl net.ipv4.tcp_max_syn_backlog 65535
systemControl net.ipv4.ip_local_port_range "1024 65535"
systemControl net.ipv4.tcp_tw_reuse 1
systemControl net.ipv4.tcp_tw_recycle 1
ResourceLimitTo 65535
