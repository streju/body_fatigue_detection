#pragma once

#include <QDebug>
#include <QImage>
#include <QString>
#include <opencv2/opencv.hpp>

#include "proto_include/alerts.pb.h"
#include "proto_include/image.pb.h"

#include "utils/Alert.hpp"
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

inline QString alertMsgFromType(const ::alerts::AlertType &protoAlertType)
{
    switch (protoAlertType)
    {
    case ::alerts::AlertType::ShoulderPose:
        return "Uncomfortable shoulders position";
    default:
        qWarning() << "Cannot translate proto AlertType: " << protoAlertType
                   << ". Returning internal translation alert.";
        return "Internal error of alert type translation";
    }
}

inline Alert toCustomAlert(const ::alerts::Alert *protoAlert)
{
    return {alertMsgFromType(protoAlert->type()), protoAlert->action() == ::alerts::Action::Start};
}

} // namespace translation
} // namespace utils
} // namespace visualization
