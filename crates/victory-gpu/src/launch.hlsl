// Adapted from Kelvaroq patch 0041, e062a82.
// Copyright (c) 2026 Stellarr Studio SAS and contributors.
// SPDX-License-Identifier: GPL-3.0-or-later
// Build: dxc -nologo -T as_6_6 -E main -Fo launch.dxil launch.hlsl
struct Payload { uint unused; };
groupshared Payload payload;
cbuffer Launch : register(b0, space9) { uint direct; };
ByteAddressBuffer record : register(t0, space9);
RWByteAddressBuffer budget : register(u0, space9);
static const uint BUDGET = 1u << 27;
[numthreads(1, 1, 1)]
void main() {
    uint3 g = uint3(0, 0, 0);
    if (direct != 0xffffffffu) {
        g = uint3(min(direct, 65535u), 1u, 1u);
    } else {
        uint3 asked = record.Load3(0);
        g = uint3(min(asked.x, 65535u), min(asked.y, 1u), min(asked.z, 1u));
        uint cost = g.x * g.y * g.z * 128u;
        bool fit = true;
        if (cost > 0) {
            uint before;
            budget.InterlockedOr(0, 0, before);
            fit = false;
            [allow_uav_condition]
            while (before < BUDGET) {
                uint next = (BUDGET - before <= cost) ? BUDGET : (before + cost);
                uint prev;
                budget.InterlockedCompareExchange(0, before, next, prev);
                if (prev == before) {
                    fit = BUDGET - before >= cost;
                    break;
                }
                before = prev;
            }
        }
        if (!fit) g = uint3(0, 0, 0);
        uint was;
        budget.InterlockedMax(4, asked.x, was);
        budget.InterlockedAdd(8, min(asked.x, 1u << 24), was);
    }
    payload.unused = 0;
    DispatchMesh(g.x, g.y, g.z, payload);
}
