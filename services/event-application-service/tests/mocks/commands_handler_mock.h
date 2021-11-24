#include <gmock/gmock.h>

#include "commands/commands_handler.h"

class CommandsHandlerMock : public eas::commands::ICommandHandler {
 public:
  MOCK_METHOD(void, Handle, (eas::commands::CreateForm), (override));
  MOCK_METHOD(void, Handle, (eas::commands::UpdateForm), (override));
  MOCK_METHOD(void, Handle, (eas::commands::DeleteFormAndResponses),
              (override));
  MOCK_METHOD(void, Handle, (eas::commands::CreateResponse), (override));
  MOCK_METHOD(void, Handle, (eas::commands::DeleteResponse), (override));
  MOCK_METHOD(void, Handle, (eas::commands::DeleteUserResponses), (override));
};