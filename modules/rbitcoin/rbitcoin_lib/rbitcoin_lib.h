#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// 1 = true, 0 = false, -1 = skip
int8_t rbitcoin_script_eval(const uint8_t *data, size_t len, uint32_t flags,
                            size_t version);
int8_t rbitcoin_verify_script(const uint8_t *script_sig, size_t script_sig_len,
                              const uint8_t *script_pubkey,
                              size_t script_pubkey_len);

#ifdef __cplusplus
}
#endif
