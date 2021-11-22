#include "pubsub_thread_pool.h"

#include <algorithm>

#include "glog/logging.h"

namespace eas::pubsub_app {

ThreadPool::ThreadPool(int thread_pool_size) {
  threads_.reserve(thread_pool_size);
  auto cq = google::cloud::CompletionQueue();
  std::generate_n(std::back_inserter(threads_), 4, [&cq] {
    return std::thread([cq]() mutable { cq.Run(); });
  });
  completion_queue_ = std::move(cq);
}

// ThreadPool::ThreadPool(ThreadPool &&tp)
//     : completion_queue_(std::move(tp.completion_queue_)),
//       threads_(std::move(threads_)) {}

// ThreadPool &ThreadPool::operator=(ThreadPool &&tp) {
//   completion_queue_ = std::move(tp.completion_queue_);
//   threads_ = std::move(tp.threads_);
//   return *this;
// }

ThreadPool::~ThreadPool() {
  LOG(INFO) << "OWL: 1";
  completion_queue_.Shutdown();
  LOG(INFO) << "OWL: 2";
  for (auto &t : threads_) {
    if (t.joinable()) t.join();
  }
  LOG(INFO) << "OWL: 3";
}

}  // namespace eas::pubsub_app