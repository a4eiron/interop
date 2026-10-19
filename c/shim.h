#ifndef SHIM_H
#define SHIM_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SHIM_ABI_VERSION 1u

uint32_t shim_abi_version(void);

typedef enum {
    SHIM_OK = 0,
    SHIM_ERR_NULL = 1,
    SHIM_ERR_RANGE = 2,
    SHIM_ERR_INTERNAL = 3,
} shim_status;

typedef struct {
    uint64_t id;
    uint32_t kind;
    uint16_t flags;
    double weight;
} shim_record;

shim_record shim_make_record(uint64_t id, uint32_t kind, double weight);
shim_status shim_scale_record(shim_record *rec, double factor);
shim_status shim_record_kind_name(uint32_t kind, const char *out_buf,
                                  size_t out_len);

#ifdef __cplusplus
}
#endif

#endif
