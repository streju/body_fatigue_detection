get_target_property(grpc_cpp_plugin_location gRPC::grpc_cpp_plugin LOCATION)
get_target_property(protoc_location protobuf::protoc LOCATION)

file(GLOB PROTOBUF_FILELIST ${PROJECT_SOURCE_DIR}/../proto/*.proto)
foreach( proto_file ${PROTOBUF_FILELIST} )
    get_filename_component(proto_path ${proto_file} PATH)
    EXEC_PROGRAM(${protoc_location} ARGS -I=${proto_path} --grpc_out=${PROTOBUF_INC_DIR} --plugin=protoc-gen-grpc=${grpc_cpp_plugin_location} ${proto_file})
    EXEC_PROGRAM(${protoc_location} ARGS -I=${proto_path} --cpp_out=${PROTOBUF_INC_DIR} ${proto_file})
endforeach()
