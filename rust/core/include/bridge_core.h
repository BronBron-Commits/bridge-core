#pragma once
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct BridgeContext BridgeContext;

uint32_t bridge_abi_version(void);

BridgeContext* bridge_create(void);
void bridge_destroy(BridgeContext* ctx);

/* Command dispatcher
 * returns 0 on success, negative on error
 */
int32_t bridge_command(
    BridgeContext* ctx,
    uint32_t cmd,
    int64_t arg,
    int64_t* out
);

#ifdef __cplusplus
}
#endif
