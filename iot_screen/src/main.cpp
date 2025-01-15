#include <Arduino.h>
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <ESP8266WiFi.h>
#include <message_generated.h>

constexpr int SCREEN_WIDTH = 128;
constexpr int SCREEN_HEIGHT = 64;
constexpr int OLED_I2C_ADDRESS = 0x3C;

const char* ssid = "spider-worse";
const char* password = "pierniktokot";

const char* server_ip = "192.168.0.165";
const int server_port = 2699;

Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

WiFiClient client;

void init_wifi() {
  Serial.println("init wifi");
  int num_ssid = WiFi.scanNetworks();
  while (num_ssid == -1) {
    Serial.println("Couldn't get a wifi connection");
    delay(500);
    num_ssid = WiFi.scanNetworks();
  }

  Serial.println("WiFi networks found:");
  for (int i = 0; i < num_ssid; i++) {
    Serial.print(i + 1);
    Serial.print(": ");
    Serial.println(WiFi.SSID(i));
  }
  WiFi.disconnect();
  WiFi.setPhyMode(WIFI_PHY_MODE_11B);
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.println("Connecting to WiFi...");
  }
  Serial.println("Connected to the WiFi network");
  Serial.print("IP Address: ");
  Serial.println(WiFi.localIP());
}

bool connect_to_tcp() {
  Serial.println("connect to tcp");
  
  if (!client.connect(server_ip, server_port)) {
    Serial.println("Connection failed");
    return false;
  }

  Serial.println("Connected to server");
  return true;
}

void setup() {
 Serial.begin(9600);
 
 init_wifi();
 connect_to_tcp();

 Serial.println("setup oled");

 if (!display.begin(OLED_I2C_ADDRESS, OLED_I2C_ADDRESS)) {
    Serial.println(F("SSD1306 allocation failed"));
    for (;;);
 }

  display.clearDisplay();

  display.setTextSize(1);
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(0, 0);
  display.println("Now playing, twoja stara!");
  display.display();
  delay(2000);
}

void try_to_reconnect() {
  if (!client.connected()) {
    Serial.println("Reconnecting...");
    if (connect_to_tcp()) {
      Serial.println("Reconnected");
    } else {
      Serial.println("Reconnection failed");
    }
    delay(1000);
  }
}

void process_message() {
  uint8_t buffer[1056];
  size_t bytes_read = client.read(buffer, sizeof(buffer));

  if (bytes_read == 0) {
    Serial.println("No data received; can't process message");
    return;
  }

  Serial.printf("Received %d bytes:\n", bytes_read);
  for (size_t i = 0; i < bytes_read; i++) {
    Serial.printf("%02x ", buffer[i]);
  }
  Serial.println();

  auto message = ScreenIoT::GetMessage(buffer);
  auto app = message->app()->c_str();
  auto payload = message->payload()->c_str();

  Serial.printf("App: %s\n", app);

  if (app == "PING") {
    Serial.println("Received PING message");
    return;
  }

  Serial.printf("App: %s\n", app);
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(0, 0);
  display.println(app);

  Serial.printf("Payload: %s\n", payload);
  display.println(payload);

  display.display();
}

void loop() {
  try_to_reconnect();

  if (client.available()) {
    Serial.println("Processing message");
    process_message();
  }

  delay(500);
}
