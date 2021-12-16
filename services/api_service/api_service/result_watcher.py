from api_service.pubsub import PubSubManager
import json
from collections import OrderedDict
import threading

class ResultWatcher:
    def __init__(self, manager, subscription, id_getter = lambda m : m.message_id) -> None:
        self.MAX_ITEMS = 1000
        self._mutexes = OrderedDict()
        self._results = OrderedDict()
        self._get_id = id_getter
        threading.Thread(target = manager.setup_subscriber, args=[subscription, lambda m : self._message_handler(m)]).start()

    def _message_handler(self, message):
        message.ack()
        print(f"Received message : {message}")
        while len(self._results) > self.MAX_ITEMS:
            self._results.popitem(False)
        id = self._get_id(message)
        self._results[id] = json.loads(message.data.decode("utf-8"))
        self._mutexes.pop(id).release()

    def add_watcher(self, mutex : threading.Lock, message_id):
        mutex.acquire()
        while len(self._mutexes) > self.MAX_ITEMS:
            self._mutexes.popitem(False).release()
        self._mutexes[message_id] = mutex

    def get_result(self, message_id):
        return self._results.pop(message_id, None)
