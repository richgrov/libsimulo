#include <emscripten.h>

extern "C" {

static std::unique_ptr<Game> root_object;

static float simulo__pose_data[17 * 2] = {0};
static float simulo__transform_data[16] = {0};

EMSCRIPTEN_KEEPALIVE
void simulo__start() {
  kSolidTexture = std::numeric_limits<uint32_t>::max();
  simulo_set_buffers(simulo__pose_data, simulo__transform_data);

  root_object = Game::create();
  simulo_set_root(root_object->simulo__id, root_object.get());
}

EMSCRIPTEN_KEEPALIVE
void simulo__update(void *ptr, float delta) {
  Object *object = static_cast<Object *>(ptr);
  object->update(delta);
}

EMSCRIPTEN_KEEPALIVE
void simulo__recalculate_transform(void *ptr) {
  Object *object = static_cast<Object *>(ptr);
  glm::mat4 transform = object->recalculate_transform();
  std::memcpy(simulo__transform_data, glm::value_ptr(transform),
              sizeof(simulo__transform_data));
}

EMSCRIPTEN_KEEPALIVE
void simulo__pose(int id, bool alive) {
  root_object->on_pose(id, alive ? std::optional<Pose>(Pose(simulo__pose_data))
                                 : std::nullopt);
}

EMSCRIPTEN_KEEPALIVE
void simulo__drop(void *ptr) {
  Object *object = static_cast<Object *>(ptr);
  delete object;
}
}