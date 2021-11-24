#include "pubsub_thread_pool.h"

#include <algorithm>

#include "glog/logging.h"

namespace eas::pubsub_controller {

ThreadPool::ThreadPool(int thread_pool_size) : pool_(thread_pool_size) {
  std::generate_n(pool_.begin(), pool_.size(), [this] {
    return std::thread([](google::cloud::CompletionQueue cq) { cq.Run(); },
                       cq_);
  });
}

ThreadPool::~ThreadPool() {
  cq_.Shutdown();
  for (auto& t : pool_) t.join();
  pool_.clear();
}

}  // namespace eas::pubsub_controller