#ifndef PUBSUB_COMMUNICATION_PUBSUB_THREAD_POOL_H
#define PUBSUB_COMMUNICATION_PUBSUB_THREAD_POOL_H

#include <thread>

#include "google/cloud/completion_queue.h"

namespace eas::pubsub_app {

class ThreadPool {
 public:
  ThreadPool(int thread_pool_size);
  ~ThreadPool();
  ThreadPool(ThreadPool const &) = delete;
  ThreadPool &operator=(ThreadPool const &) = delete;
  ThreadPool(ThreadPool &&) = default;
  ThreadPool &operator=(ThreadPool &&) = default;

  google::cloud::CompletionQueue &CompletionQueue() & {
    return completion_queue_;
  }

 private:
  google::cloud::CompletionQueue completion_queue_;
  std::vector<std::thread> threads_;
};

}  // namespace eas::pubsub_app

#endif  // PUBSUB_COMMUNICATION_PUBSUB_THREAD_POOL_H