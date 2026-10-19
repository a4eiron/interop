#include "shim.h"
#include <string.h>

uint32_t shim_abi_version(void) {
    return SHIM_ABI_VERSION;
}

shim_record shim_make_record(uint64_t id, uint32_t kind, double weight) {
    shim_record r = {.id = id, .kind = kind, .flags = 0, .weight = weight};
    return r;
}

shim_status shim_scale_record(shim_record *rec, double factor) {
    if (!rec)
        return SHIM_ERR_NULL;
    if (factor < 0.0)
        return SHIM_ERR_RANGE;
    rec->weight *= factor;
    return SHIM_OK;
}

shim_status shim_record_kind_name(uint32_t kind, const char *out_buf,
                                  size_t out_len) {
    if (!out_buf)
        return SHIM_ERR_NULL;
    const char *name;
    switch (kind) {
    case 1:
        name = "scalar";
        break;
    case 2:
        name = "tensor";
        break;
    default:
        return SHIM_ERR_RANGE;
    }

    size_t n = strlen(name);

    if (out_len < n + 1)
        return SHIM_ERR_RANGE;

    memcpy((char *)out_buf, name, n + 1);
    return SHIM_OK;
}
