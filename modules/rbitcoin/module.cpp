#include "module.h"
#include "rbitcoin_lib/rbitcoin_lib.h"

#include <bitcoinfuzz/result.h>

namespace bitcoinfuzz {
namespace module {

Rbitcoin::Rbitcoin(void) : BaseModule("Rbitcoin") {}

namespace {
std::optional<bool> FromI8(int8_t v) {
  if (v < 0)
    return Skip();
  return v != 0;
}
} // namespace

std::optional<bool> Rbitcoin::script_eval(const std::vector<uint8_t> &input_data,
                                          unsigned int flags,
                                          size_t version) const {
  if (input_data.empty())
    return Skip();
  return FromI8(rbitcoin_script_eval(input_data.data(), input_data.size(),
                                     flags, version));
}

std::optional<bool>
Rbitcoin::verify_script(const std::vector<uint8_t> &script_sig,
                        const std::vector<uint8_t> &script_pubkey) const {
  if (script_sig.empty() || script_pubkey.empty())
    return Skip();
  return FromI8(rbitcoin_verify_script(script_sig.data(), script_sig.size(),
                                       script_pubkey.data(),
                                       script_pubkey.size()));
}

} // namespace module
} // namespace bitcoinfuzz
