#include <glog/logging.h>

#include <iostream>

#include "application.h"

int main(int argc, char* argv[]) {
  FLAGS_colorlogtostderr = 1;
  FLAGS_logtostderr = 1;

  eas::Application app(argc, argv);

  return app.Execute();
}