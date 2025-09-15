#include <emscripten.h>

#include <memory>

#include "simulo/simulo.h"

using namespace simulo;

static std::unique_ptr<PoseHandler> root_object;

static float simulo__pose_data[17 * 2] = {0};
static float simulo__transform_data[16] = {0};

__attribute__((__import_name__("simulo_random"))) extern float
simulo_random(void);

__attribute__((__import_name__("simulo_window_width"))) extern int32_t
simulo_window_width(void);

__attribute__((__import_name__("simulo_window_height"))) extern int32_t
simulo_window_height(void);

__attribute__((__import_name__("simulo_set_buffers"))) extern void
simulo_set_buffers(float *pose, float *transform);

__attribute__((__import_name__("simulo_set_root"))) extern void
simulo_set_root(uint32_t id, void *self);

glm::ivec2 simulo::window_size() {
  return glm::ivec2(simulo_window_width(), simulo_window_height());
}

float simulo::random_float() { return simulo_random(); }

void simulo::start(std::unique_ptr<PoseHandler> root) {
  root_object = std::move(root);

  simulo_set_buffers(simulo__pose_data, simulo__transform_data);
  simulo_set_root(root_object->simulo__id, root_object.get());
}

extern "C" {

EMSCRIPTEN_KEEPALIVE
void simulo__update(void *ptr, float delta) {
  Object *object = static_cast<Object *>(ptr);
  object->update(delta);
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
