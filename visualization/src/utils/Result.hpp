#pragma once

#include <memory>
#include <utility>
#include <variant>

namespace visualization
{
namespace utils
{

struct None
{
};

template <typename T>
struct Ok
{
    Ok(const T &ok) : val(ok) {}
    T val;
};

template <typename T>
struct Err
{
    Err(const T &err) : val(err) {}
    T val;
};

template <typename OkT, typename ErrT>
class Result
{
public:
    Result(const Ok<OkT> &ok) : data(ok) {}
    Result(const Err<ErrT> &err) : data(err) {}
    bool isOk() const { return std::get_if<Ok<OkT>>(&data) != nullptr; }
    bool isErr() const { return std::get_if<Err<ErrT>>(&data) != nullptr; }
    std::unique_ptr<OkT> getOk() const
    {
        if (const auto ok = std::get_if<Ok<OkT>>(&data))
        {
            return std::make_unique<OkT>(std::move(ok->val));
        }
        return nullptr;
    }
    std::unique_ptr<ErrT> getErr() const
    {
        if (const auto err = std::get_if<Err<ErrT>>(&data))
        {
            return std::make_unique<ErrT>(std::move(err->val));
        }
        return nullptr;
    }

private:
    std::variant<Ok<OkT>, Err<ErrT>> data;
};

using CommonResult = Result<None, std::string>;
} // namespace utils
} // namespace visualization