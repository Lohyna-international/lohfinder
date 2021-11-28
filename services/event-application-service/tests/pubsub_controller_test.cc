#include "pubsub_controller/pubsub_controller.h"

#include <gtest/gtest.h>

#include "glog/logging.h"
#include "google/cloud/pubsub/mocks/mock_ack_handler.h"
#include "google/cloud/pubsub/publisher_options.h"
#include "mocks/commands_handler_mock.h"
#include "mocks/connection_factory_mock.h"
#include "mocks/query_handler_mock.h"
#include "pubsub_controller/connection_factory.h"
#include "pubsub_controller/pubsub_subscribers.h"

using namespace eas;
using namespace eas::pubsub_controller;
using namespace eas::commands;
using namespace eas::queries;
using namespace ::testing;
using namespace google::cloud::pubsub;
using namespace google::cloud::pubsub_mocks;
using namespace google::cloud;

class PubSubEnv : public Environment {
 public:
  void SetUp() override {
    FLAGS_alsologtostderr = true;
    FLAGS_colorlogtostderr = true;
    google::InitGoogleLogging("afs_tests");
  }
  void TearDown() override { google::ShutdownGoogleLogging(); }
};

class PubSubTests : public Test {
 protected:
  void SetUp() override {
    conn_factory_ = std::make_unique<MockConnectionFactory>();
    p_conn_factory_ = conn_factory_.get();
    query_handler_ = std::make_shared<QueryHandlerMock>();
    cmd_handler_ = std::make_shared<CommandsHandlerMock>();
    controller_ = std::make_unique<PubSubController>(
        "test", std::move(conn_factory_), query_handler_, cmd_handler_);
  }

  void TearDown() override {
    conn_factory_.reset(nullptr);
    controller_.reset(nullptr);
  }

  std::unique_ptr<MockConnectionFactory> conn_factory_;
  utils::non_owning<MockConnectionFactory*> p_conn_factory_;
  std::shared_ptr<QueryHandlerMock> query_handler_;
  std::shared_ptr<CommandsHandlerMock> cmd_handler_;
  std::unique_ptr<PubSubController> controller_;
};

struct PublisherAction {
  static int i;
  future<Status> operator()(SubscriberConnection::SubscribeParams params) {
    auto generator =
        [](promise<google::cloud::Status> promise,
           pubsub::SubscriberConnection::SubscribeParams const& params) {
          auto mock_handler = absl::make_unique<MockAckHandler>();
          EXPECT_CALL(*mock_handler, ack_id)
              .WillRepeatedly(Return("ack-id-" + std::to_string(i)));
          EXPECT_CALL(*mock_handler, ack);
          params.callback(pubsub::MessageBuilder{}
                              .SetData("message-" + std::to_string(i))
                              .SetAttribute("sender", "aes")
                              .Build(),
                          pubsub::AckHandler(std::move(mock_handler)));
          ++i;
          // Close the stream with a successful error code
          promise.set_value({});
        };
    promise<Status> p;
    auto result = p.get_future();
    // start the generator in a separate thread.
    (void)std::async(std::launch::async, generator, std::move(p),
                     std::move(params));
    return result;
  }
};

int PublisherAction::i = 0;

::testing::Environment* const env =
    ::testing::AddGlobalTestEnvironment(new PubSubEnv);

TEST_F(PubSubTests, TestPublishSubscribe) {
  EXPECT_CALL(*cmd_handler_, Handle(An<CreateForm>()));
  EXPECT_CALL(*cmd_handler_, Handle(An<CreateResponse>()));
  EXPECT_CALL(*cmd_handler_, Handle(An<UpdateForm>()));
  EXPECT_CALL(*cmd_handler_, Handle(An<DeleteFormAndResponses>()));
  EXPECT_CALL(*cmd_handler_, Handle(An<DeleteUserResponses>()));
  EXPECT_CALL(*cmd_handler_, Handle(An<DeleteResponse>()));

  EXPECT_CALL(*query_handler_, Execute(An<FormQuery>()));
  EXPECT_CALL(*query_handler_, Execute(An<UserResponseQuery>()));
  EXPECT_CALL(*query_handler_, Execute(An<FormResponsesQuery>()));

  EXPECT_CALL(*p_conn_factory_->pub_, Publish).Times(3);

  EXPECT_CALL(*p_conn_factory_->sub_, Subscribe)
      .WillRepeatedly(PublisherAction());

  controller_->Start();
  controller_->Shutdown();
}