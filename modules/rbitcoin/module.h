#pragma once

#include <bitcoinfuzz/basemodule.h>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

namespace bitcoinfuzz {
namespace module {
class Rbitcoin : public BaseModule {
public:
  Rbitcoin(void);
  std::optional<bool> script_eval(const std::vector<uint8_t> &input_data,
                                  unsigned int flags,
                                  size_t version) const override;
  std::optional<bool>
  verify_script(const std::vector<uint8_t> &script_sig,
                const std::vector<uint8_t> &script_pubkey) const override;
  ~Rbitcoin() noexcept override = default;
};
} // namespace module
} // namespace bitcoinfuzz
