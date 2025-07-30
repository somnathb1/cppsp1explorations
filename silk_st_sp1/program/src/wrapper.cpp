#include <silkworm/dev/state_transition.hpp>
#include <nlohmann/json.hpp>

const static std::string json_str = R"json(
{"add": {"_info": {"comment": "Ori Pomerantz qbzzt1@gmail.com","filling-rpc-server": "evm version 1.11.4-unstable-e14043db-20230308","filling-tool-version": "retesteth-0.3.0-shanghai+commit.fd2c0a83.Linux.g++","generatedTestHash": "f486c80e808d34507133961cbd17c5e0f9ec049879dd3f7cc78a9eb55ac63226","labels": {"0": "add_neg1_neg1","1": "add_neg1_4","2": "add_neg1_1","3": "add_0_0","4": "add_1_neg1"},"lllcversion": "Version: 0.5.14-develop.2022.7.30+commit.a096d7a9.Linux.g++","solidity": "Version: 0.8.17+commit.8df45f5f.Linux.g++","source": "src/GeneralStateTestsFiller/VMTests/vmArithmeticTest/addFiller.yml","sourceHash": "78afea990a2d534831acc4883b9ff6e81d560091942db7234232d68fdbf1c33e"},"env": {"currentBaseFee": "0x0a","currentCoinbase": "0x2adc25665018aa1fe0e6bc666dac8fc2697ff9ba","currentDifficulty": "0x020000","currentGasLimit": "0x05f5e100","currentNumber": "0x01","currentRandom": "0x0000000000000000000000000000000000000000000000000000000000020000","currentTimestamp": "0x03e8","previousHash": "0x5e20a0453cecd065ea59c37ac63e079ee08998b6045136a8ce6635c7912ec0b6"},"post": {"Shanghai": [{"hash": "0x6e9dccb57a15e2885ff1193da0db98cbaaac218bf3a0abeb0c3ceff966de2830","indexes": {"data": 0,"gas": 0,"value": 0},"logs": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","txbytes": "0xf885800a8404c4b40094cccccccccccccccccccccccccccccccccccccccc01a4693c613900000000000000000000000000000000000000000000000000000000000000001ba0e8ff56322287185f6afd3422a825b47bf5c1a4ccf0dc0389cdc03f7c1c32b7eaa0776b02f9f5773238d3ff36b74a123f409cd6420908d7855bbe4c8ff63e00d698"},{"hash": "0x1a3420dfb2280397c1b81ff159bd4d6eddc12d7e333e82a01fd4afafad3b2ae4","indexes": {"data": 1,"gas": 0,"value": 0},"logs": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","txbytes": "0xf885800a8404c4b40094cccccccccccccccccccccccccccccccccccccccc01a4693c613900000000000000000000000000000000000000000000000000000000000000011ba02c5e81a024dd0f6fb773c8787fa46ab5eb55cb73df83562e6ddbe9106a3df7f6a029437b9a23e45bbfce086f2ddaa98b1e9e6914d7e58e2c5a128310042b332f89"},{"hash": "0x416be8cb4f40d5a29ed56578cf776c5198e58c181ab3534a1094df5f7f61fb02","indexes": {"data": 2,"gas": 0,"value": 0},"logs": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","txbytes": "0xf885800a8404c4b40094cccccccccccccccccccccccccccccccccccccccc01a4693c613900000000000000000000000000000000000000000000000000000000000000021ba0fc37ad4eb0633eb18f2b7867bacbe994a2ffcbb04a71e394e6e76041f6ce216fa03b1b415a5c386d8de9e16be9fdc188234b80a0dec99922d03c240f2e463053e3"},{"hash": "0x416be8cb4f40d5a29ed56578cf776c5198e58c181ab3534a1094df5f7f61fb02","indexes": {"data": 3,"gas": 0,"value": 0},"logs": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","txbytes": "0xf885800a8404c4b40094cccccccccccccccccccccccccccccccccccccccc01a4693c613900000000000000000000000000000000000000000000000000000000000000031ba0f06eb219c5dba98711a9a2678339f64d172bfac289a5c43a0018d3917be8dc2aa0147bd7a6ee30217e63cbddc28b0e72f115da754d8916b87992aa27ed00eb105e"},{"hash": "0x416be8cb4f40d5a29ed56578cf776c5198e58c181ab3534a1094df5f7f61fb02","indexes": {"data": 4,"gas": 0,"value": 0},"logs": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","txbytes": "0xf885800a8404c4b40094cccccccccccccccccccccccccccccccccccccccc01a4693c613900000000000000000000000000000000000000000000000000000000000000041ba0c148a101aa54703ff0e949441bdba90b1972a16c338f7f9a24b07f0313cd49d6a028cb82229b8a57e2048761d6fa5060c5b459f000d4e218de1372c1df9cfa171e"}]},"pre": {"0x0000000000000000000000000000000000000100": {"balance": "0x0ba1a9ce0ba1a9ce","code": "0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0160005500","nonce": "0x00","storage": {}},"0x0000000000000000000000000000000000000101": {"balance": "0x0ba1a9ce0ba1a9ce","code": "0x60047fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0160005500","nonce": "0x00","storage": {}},"0x0000000000000000000000000000000000000102": {"balance": "0x0ba1a9ce0ba1a9ce","code": "0x60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0160005500","nonce": "0x00","storage": {}},"0x0000000000000000000000000000000000000103": {"balance": "0x0ba1a9ce0ba1a9ce","code": "0x600060000160005500","nonce": "0x00","storage": {}},"0x0000000000000000000000000000000000000104": {"balance": "0x0ba1a9ce0ba1a9ce","code": "0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60010160005500","nonce": "0x00","storage": {}},"0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b": {"balance": "0x0ba1a9ce0ba1a9ce","code": "0x","nonce": "0x00","storage": {}},"0xcccccccccccccccccccccccccccccccccccccccc": {"balance": "0x0ba1a9ce0ba1a9ce","code": "0x600060006000600060006004356101000162fffffff100","nonce": "0x00","storage": {}}},"transaction": {"data": ["0x693c61390000000000000000000000000000000000000000000000000000000000000000","0x693c61390000000000000000000000000000000000000000000000000000000000000001","0x693c61390000000000000000000000000000000000000000000000000000000000000002","0x693c61390000000000000000000000000000000000000000000000000000000000000003","0x693c61390000000000000000000000000000000000000000000000000000000000000004"],"gasLimit": ["0x04c4b400"],"gasPrice": "0x0a","nonce": "0x00","secretKey": "0x45a915e4d060149eb4365960e6a7a45f334393093061116b197e3240065ff2d8","sender": "0xa94f5374fce5edbc8e2a8697c15331677e6ebf0b","to": "0xcccccccccccccccccccccccccccccccccccccccc","value": ["0x01"]}}}
)json";


// using namespace ;

// const state_transition

// class Dog {
//     private:
//     int lives;
//     std::string petName;

//     nlohmann::json test_data_;
//     std::string test_name_;
//     unsigned total_count_{};
//     unsigned failed_count_{};
//     bool terminate_on_error_{false};
//     bool show_diagnostics_{false};

//     public:
//     explicit Dog(const bool terminate_on_error, const bool show_diagnostics) noexcept
//         : terminate_on_error_{terminate_on_error},
//         show_diagnostics_{show_diagnostics} {
//     }

//     explicit Dog(const std::string& file_path) noexcept {

//     }
//     explicit Dog(const std::string& json_str, bool terminate_on_error, bool show_diagnostics) noexcept {

//     }
//     int getLives() {
//         return lives;
//     }
//     void doStuff() {
//         if (show_diagnostics_) {
//             test_name_  = "moha";
//         }
//     }
// };

extern "C" void sample_run_wrapped() {
    // std::string json_str2 = std::make_unique<string>;
    auto state_transition = silkworm::cmd::state_transition::StateTransition(false, true);
    // auto state_transition = silkworm::cmd::state_transition::Dog(false, true);
    state_transition.run();
    // auto doggie = Dog(false, true);
    // doggie.doStuff();
    return;
}
