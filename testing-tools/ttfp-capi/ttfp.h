#ifndef TTFP_H
#define TTFP_H

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ttfp_font ttfp_font;

typedef uint32_t ttfp_tag;

typedef struct ttfp_rect {
    int16_t x_min;
    int16_t y_min;
    int16_t x_max;
    int16_t y_max;
} ttfp_rect;

typedef struct ttfp_variation_axis {
    ttfp_tag tag;
    float min_value;
    float default_value;
    float max_value;
    uint16_t name_id;
    bool hidden;
} ttfp_variation_axis;

typedef void (*ttfp_outline_builder_move_to_fn)(float x, float y, void *data);
typedef void (*ttfp_outline_builder_line_to_fn)(float x, float y, void *data);
typedef void (*ttfp_outline_builder_quad_to_fn)(float x1, float y1, float x, float y, void *data);
typedef void (*ttfp_outline_builder_curve_to_fn)(float x1, float y1, float x2, float y2, float x, float y, void *data);
typedef void (*ttfp_outline_builder_close_path_fn)(void *data);

typedef struct ttfp_outline_builder {
    ttfp_outline_builder_move_to_fn move_to;
    ttfp_outline_builder_line_to_fn line_to;
    ttfp_outline_builder_quad_to_fn quad_to;
    ttfp_outline_builder_curve_to_fn curve_to;
    ttfp_outline_builder_close_path_fn close_path;
} ttfp_outline_builder;

void ttfp_init_log();

ttfp_font* ttfp_font_create(const char *data,
                            uint32_t data_size,
                            uint32_t index);

void ttfp_font_destroy(ttfp_font *font);

bool ttfp_outline_glyph(ttfp_font *font,
                        ttfp_outline_builder builder,
                        void* user_data,
                        uint16_t glyph_id,
                        ttfp_rect *bbox);

bool ttfp_outline_variable_glyph(ttfp_font *font,
                                 ttfp_outline_builder builder,
                                 void* user_data,
                                 uint16_t glyph_id,
                                 const int32_t *coordinates,
                                 uint32_t coordinates_size,
                                 ttfp_rect *bbox);

int16_t ttfp_ascender(const ttfp_font *font);
int16_t ttfp_height(const ttfp_font *font);
uint16_t ttfp_number_of_glyphs(const ttfp_font *font);

uint16_t ttfp_variation_axes_count(const ttfp_font *font);

bool ttfp_get_variation_axis(const ttfp_font *font,
                             uint16_t index,
                             ttfp_variation_axis *axis);

bool ttfp_get_variation_axis_by_tag(const ttfp_font *font,
                                    ttfp_tag tag,
                                    ttfp_variation_axis *axis);

bool ttfp_map_variation_coordinates(const ttfp_font *font,
                                    int32_t *coordinates,
                                    uint32_t coordinates_size);

#ifdef __cplusplus
}
#endif

#endif /* TTFP_H */
