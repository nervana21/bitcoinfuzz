// Local smoke for the rbitcoin module without Bitcoin Core.
// Build: make poc-smoke
#include "module.h"

#include <cassert>
#include <cstdio>
#include <vector>

int main() {
  bitcoinfuzz::module::Rbitcoin mod;

  // OP_TRUE — execution succeeds
  {
    const std::vector<uint8_t> script{0x51};
    auto r = mod.script_eval(script, 0, 0);
    assert(r.has_value());
    assert(*r);
  }

  // OP_RETURN — execution fails
  {
    const std::vector<uint8_t> script{0x6a};
    auto r = mod.script_eval(script, 0, 0);
    assert(r.has_value());
    assert(!*r);
  }

  // Empty scriptSig / OP_TRUE scriptPubKey — skip (empty sig)
  {
    auto r = mod.verify_script({}, {0x51});
    assert(!r.has_value());
  }

  // scriptSig OP_1 + scriptPubKey OP_0 → top false → reject
  {
    const std::vector<uint8_t> script_sig{0x51};
    const std::vector<uint8_t> script_pubkey{0x00};
    auto r = mod.verify_script(script_sig, script_pubkey);
    assert(r.has_value());
    assert(!*r);
  }

  // scriptSig OP_1 + scriptPubKey OP_1 → true on stack → accept
  {
    const std::vector<uint8_t> script_sig{0x51};
    const std::vector<uint8_t> script_pubkey{0x51};
    auto r = mod.verify_script(script_sig, script_pubkey);
    assert(r.has_value());
    assert(*r);
  }

  std::puts("rbitcoin poc_smoke: ok");
  return 0;
}
