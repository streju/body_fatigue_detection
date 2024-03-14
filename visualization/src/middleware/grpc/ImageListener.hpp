#pragma once

#include <string>

#include <QThread>

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>

#include "proto_include/common.pb.h"
#include "proto_include/image.pb.h"
#include "proto_include/visualization_service.grpc.pb.h"

#include "utils/Image.hpp"

namespace visualization
{
namespace middleware
{

class ImageListener : public QThread
{
    Q_OBJECT
public:
    ImageListener(const std::string &addr);
    ImageListener(const ImageListener &) = delete;
    ImageListener(ImageListener &&) = delete;
    ImageListener &operator=(const ImageListener &) = delete;
    ImageListener &operator=(ImageListener &&) = delete;

protected:
    void run() override;

signals:
    void CameraFrameNotif(const utils::Image &);

private:
    class Impl : public visualization::VisualizationService::Service
    {
        ImageListener &parent_;

    public:
        Impl(ImageListener &parent) : parent_(parent) {}
        grpc::Status HandleImageNotif(grpc::ServerContext *contex,
                                      const ::img_common::CameraFrame *request,
                                      ::common::Result *response) override;
    };

    void emitCameraFrameNotif(const utils::Image &img);
    std::string addr_;
};

} // namespace middleware
} // namespace visualization
