enable_testing()
find_package(GTest REQUIRED)

set(tst_sources ${tst_sources}
)

add_executable(${PROJECT_NAME}-ut ${tst_sources})
target_link_libraries(${PROJECT_NAME}-ut GTest::gtest GTest::gtest_main ffd_lib)