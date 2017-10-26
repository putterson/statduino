#include <math.h> 
#include <Adafruit_CircuitPlayground.h>

#define ALARM_FREQ 640
#define ALARM_SPEED 100
#define ALARM_DUTY 80

#define ANIM_SPEED 0.001;

enum buildstatus {
  success,
  failure,
  building,
  unknown,
};

struct color {
  byte r;
  byte g;
  byte b;
};

typedef void (* step_light_anim_func) ();

void null_animate() {
  return;
}

color sawtooth(color c, float pos) {
  return (color){
    c.r * pos,
    c.g * pos,
    c.b * pos
  };
}

color rev_sawtooth(color c, float pos) {
  return (color){
    c.r * (1.0-pos),
    c.g * (1.0-pos),
    c.b * (1.0-pos)
  };
}

color square(color c, float pos) {
  color s = (color){0,0,0};
  if(pos <= 0.3){
    return c;
  }
  return s;
}

//Globals
buildstatus G_buildstatus = unknown;
float lightphase = 0;
step_light_anim_func F_light_animate = null_animate;


void alarm(int cycles) {
  if (!CircuitPlayground.slideSwitch()) {
    return;
  }
  for(int i=0; i < cycles; i++){
    if (CircuitPlayground.slideSwitch()) {
      CircuitPlayground.playTone(ALARM_FREQ, ALARM_DUTY);
      delay(ALARM_SPEED - ALARM_DUTY);
    }
  }
}

void build_success() {
  for(int i = 0; i < 5; i++){
    CircuitPlayground.strip.setPixelColor(i, 0, 50, 0);
  }  
  CircuitPlayground.strip.show();
}

void building_anim() {
  color c = (color){ 0, 0, 120};
  for(int i = 0; i < 5; i++){
    color s = square(c, fmod((i / 5.0) + lightphase, 1.0));
    CircuitPlayground.strip.setPixelColor(i, s.r, s.g, s.b);
  }  
  CircuitPlayground.strip.show();
}

void failure_anim() {
  color c = (color){ 120, 0, 0};
  color s = rev_sawtooth(c, lightphase);
  for(int i = 0; i < 5; i++){
    CircuitPlayground.strip.setPixelColor(i, s.r, s.g, s.b);
  }  
  CircuitPlayground.strip.show();
}

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
  CircuitPlayground.begin();
  build_success();
  F_light_animate = building_anim;
}

void loop() {
  F_light_animate();
  lightphase += ANIM_SPEED;
  if (lightphase > 1.0) {
    lightphase = lightphase - 1.0;
  }
}
