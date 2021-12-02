#ifndef PUBSUB_CONTROLLER_PUBSUB_THREAD_POOL_H
#define PUBSUB_CONTROLLER_PUBSUB_THREAD_POOL_H

#include <thread>

#include "google/cloud/completion_queue.h"

namespace eas::pubsub_controller {

class ThreadPool final {
 public:
  ThreadPool(int thread_pool_size);
  ~ThreadPool();

  google::cloud::CompletionQueue cq() const { return cq_; }

 private:
  google::cloud::CompletionQueue cq_;
  std::vector<std::thread> pool_;
};

}  // namespace eas::pubsub_controller

#endif  // PUBSUB_CONTROLLER_PUBSUB_THREAD_POOL_H
