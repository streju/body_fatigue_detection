#include "ImageListener.hpp"

#include <QDebug>

#include "utils/ProtoTranslation.hpp"

namespace visualization
{
namespace middleware
{

ImageListener::ImageListener(const std::string &addr) : addr_(addr) {}

void ImageListener::run()
{
    Impl service{*this};
    grpc::EnableDefaultHealthCheckService(true);
    grpc::reflection::InitProtoReflectionServerBuilderPlugin();
    grpc::ServerBuilder builder;

    builder.AddListeningPort(addr_, grpc::InsecureServerCredentials());
    builder.RegisterService(&service);
    std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
    qDebug() << "[ImageListener] Start listening on " << QString(addr_.c_str());
    server->Wait();
}

void ImageListener::emitCameraFrameNotif(const utils::Image &img)
{
    emit CameraFrameNotif(img);
}

void ImageListener::emitAlertNotif(const utils::Alert &alert)
{
    emit CameraAlertNotif(alert);
}

grpc::Status ImageListener::Impl::HandleImageNotif(grpc::ServerContext *contex,
                                                   const ::img_common::CameraFrame *request,
                                                   ::common::Result *response)
{
    qInfo() << "Got Image. Image height: " << request->image().height() << " , width: " << request->image().width();
    parent_.emitCameraFrameNotif(utils::translation::toCustomImage(request->image()));
    response->set_status(::common::Status::Succeeded);
    return grpc::Status::OK;
}

grpc::Status ImageListener::Impl::HandleAlertNotif(grpc::ServerContext *contex,
                                                   const ::alerts::Alert *request,
                                                   ::common::Result *response)
{
    qInfo() << "Got AlertNotif";
    parent_.emitAlertNotif(utils::translation::toCustomAlert(request));
    response->set_status(::common::Status::Succeeded);
    return grpc::Status::OK;
}

} // namespace middleware
} // namespace visualization