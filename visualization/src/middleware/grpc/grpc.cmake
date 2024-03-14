if (BUILD_GRPC)
	set(GRPC_WARNINGS_DISABLING "-Wno-comments -Wno-attributes -Wno-unused-function -Wno-redundant-move \
	-Wno-sign-compare -Wno-type-limits -Wno-class-memaccess -Wno-extra -Wno-return-type -Wno-deprecated -Wno-deprecated-declarations")
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${GRPC_WARNINGS_DISABLING}")
	include(FetchContent)
	FetchContent_Declare(
		gRPC
		GIT_REPOSITORY https://github.com/grpc/grpc
		GIT_TAG        v1.61.0
	)
	set(FETCHCONTENT_QUIET OFF)
	set(ABSL_ENABLE_INSTALL ON)
	FetchContent_MakeAvailable(gRPC)
else()
	set(GRPC_PREFIX_PATH "${PROJECT_SOURCE_DIR}/deps/grpc/install")
	list(APPEND CMAKE_PREFIX_PATH ${CMAKE_PREFIX_PATH} ${GRPC_PREFIX_PATH})
	
	find_package(absl CONFIG REQUIRED)
	find_package(Protobuf REQUIRED CONFIG)
	find_package(gRPC REQUIRED CONFIG)

	find_package(PkgConfig REQUIRED)
	pkg_check_modules(LIBZIP REQUIRED zlib)

	include(middleware/grpc/protobuf.cmake)
	list(APPEND LINKED_LIBS ${LINKED_LIBS} gRPC::grpc++ gRPC::grpc++_reflection protobuf::libprotobuf)
endif()
