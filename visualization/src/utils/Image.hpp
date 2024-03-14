#pragma once

#include <QImage>

namespace visualization
{
namespace utils
{

enum class ImgType
{
    Raw = 0,
    DebugLandmarks = 1
};

struct Image
{
    Image(const QImage &data, const ImgType type) : data(data), type(type) {}
    QImage data;
    ImgType type;
};
} // namespace utils
} // namespace visualization