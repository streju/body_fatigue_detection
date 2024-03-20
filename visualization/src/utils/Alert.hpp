#pragma once

#include <QString>

namespace visualization
{
namespace utils
{

struct Alert
{
    Alert(const QString &msg, bool isStart) : msg(msg), isStart(isStart) {}
    QString msg;
    bool isStart;
};
} // namespace utils
} // namespace visualization