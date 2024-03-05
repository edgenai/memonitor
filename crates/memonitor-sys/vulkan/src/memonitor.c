#include <volk.h>

#include <memonitor.h>

int init_vk() {
    if (volkInitialize() == VK_SUCCESS) {
        return 0;
    } else {
        return -1;
    }
}

void term_vk() {
    volkFinalize();
}
