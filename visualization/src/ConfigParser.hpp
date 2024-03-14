#pragma once

#include <fstream>
#include <memory>
#include <string>

#include <nlohmann/json.hpp>

#include "utils/Result.hpp"

namespace visualization
{

using json = nlohmann::json;

class ConfigParser
{
public:
    utils::CommonResult parseJsonFile(const std::string &jsonConfigFile)
    {
        std::ifstream configFile(jsonConfigFile);
        if (!configFile)
        {
            return utils::Err{std::string("[ConfigParser] Cannot open jsonConfigFile: " + jsonConfigFile)};
        }

        std::unique_ptr<json> configParsedJson;
        try
        {
            configParsedJson = std::make_unique<json>(json::parse(configFile));
        }
        catch (const std::exception &e)
        {
            return utils::Err{std::string("[ConfigParser] Failed to parse JSON file: ") + std::string(e.what())};
        }

        const auto visualizationServerAddrRes =
            getFromJson<std::string>("visualization_server_addr", *configParsedJson);
        if (auto err = visualizationServerAddrRes.getErr())
        {
            return utils::Err{*err};
        }

        const auto visualizationServerPortRes = getFromJson<int>("visualization_server_port", *configParsedJson);
        if (auto err = visualizationServerPortRes.getErr())
        {
            return utils::Err{*err};
        }
        visualizationServerAddrWithPort_ = std::string(*visualizationServerAddrRes.getOk() + ":"
                                                       + std::to_string(*visualizationServerPortRes.getOk()));
        return utils::Ok(utils::None());
    }

    std::string getVisualizationServerFullAddr() const { return visualizationServerAddrWithPort_; }

private:
    template <typename T>
    utils::Result<T, std::string> getFromJson(const std::string paramName, const nlohmann::json &json) const
    {
        if (json.find(paramName) == json.end() || json.at(paramName).empty())
        {
            return utils::Err{std::string("[ConfigParser] Cannot get required config param: ") + paramName};
        }
        return utils::Ok{json.at(paramName).get<T>()};
    }
    std::string visualizationServerAddrWithPort_;
};

} // namespace visualization