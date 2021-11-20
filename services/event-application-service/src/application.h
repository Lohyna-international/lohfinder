#ifndef APPLICATION_H
#define APPLICATION_H

#include "pubsub_communication/pubsub_controller.h"

namespace eas {

class Application {
 public:
  Application(int argc, char* argv[]);
  ~Application();
  int Execute();

 private:
  int argc_;
  char** argv_;

  pubsub_app::PubSubController app_;
};

}  // namespace eas

#endif  // APPLICATION_H