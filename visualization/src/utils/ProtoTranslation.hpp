#pragma once

#include <QImage>
#include <opencv2/opencv.hpp>

#include "proto_include/image.pb.h"
#include "utils/Image.hpp"

namespace visualization
{
namespace utils
{
namespace translation
{

inline ImgType imgType(const ::img_common::ImgType &protoImgType)
{
    return protoImgType == ::img_common::ImgType::DebugLandmarks ? ImgType::DebugLandmarks : ImgType::Raw;
}

inline Image toCustomImage(const ::img_common::Image &protoImg)
{
    cv::Mat imgCv(protoImg.height(),
                  protoImg.width(),
                  CV_MAKETYPE(CV_8U, protoImg.channels()),
                  const_cast<char *>(protoImg.data().c_str()));
    return {{(uchar *)imgCv.data, imgCv.cols, imgCv.rows, qsizetype(imgCv.step), QImage::Format_RGB888},
            imgType(protoImg.type())};
}

} // namespace translation
} // namespace utils
} // namespace visualization
