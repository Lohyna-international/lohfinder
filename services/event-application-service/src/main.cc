#include <glog/logging.h>

#include <iostream>

#include "application.h"

int main(int argc, char* argv[]) {
  FLAGS_colorlogtostderr = true;
  FLAGS_alsologtostderr = true;

  eas::Application app(argc, argv);

  return app.Execute();
}
