#include <Arduino.h>
#include <Wire.h>
#define FS_NO_GLOBALS 
#include <LittleFS.h>
#include <SPI.h>
#include <Adafruit_GFX.h>
#include <Adafruit_ILI9341.h>
#include <JPEGDecoder.h>
#include <ESP8266WiFi.h>
#include <message_generated.h>

constexpr int SCREEN_WIDTH = 128;
constexpr int SCREEN_HEIGHT = 64;
constexpr int OLED_I2C_ADDRESS = 0x3C;

#define TFT_CS D8
#define TFT_RST D3
#define TFT_DC D4

const char *ssid = "spider-worse";
const char *password = "pierniktokot";

const char *server_ip = "192.168.0.165";
const int server_port = 2699;

Adafruit_ILI9341 tft = Adafruit_ILI9341(TFT_CS, TFT_DC);
WiFiClient client;
void init_wifi()
{
    Serial.println("init wifi");
    int num_ssid = WiFi.scanNetworks();
    while (num_ssid == -1)
    {
        Serial.println("Couldn't get a wifi connection");
        delay(500);
        num_ssid = WiFi.scanNetworks();
    }

    Serial.println("WiFi networks found:");
    for (int i = 0; i < num_ssid; i++)
    {
        Serial.print(i + 1);
        Serial.print(": ");
        Serial.println(WiFi.SSID(i));
    }
    WiFi.disconnect();
    WiFi.setPhyMode(WIFI_PHY_MODE_11B);
    WiFi.begin(ssid, password);
    while (WiFi.status() != WL_CONNECTED)
    {
        delay(500);
        Serial.println("Connecting to WiFi...");
    }
    Serial.println("Connected to the WiFi network");
    Serial.print("IP Address: ");
    Serial.println(WiFi.localIP());
}

bool connect_to_tcp()
{
    Serial.println("connect to tcp");

    if (!client.connect(server_ip, server_port))
    {
        Serial.println("Connection failed");
        return false;
    }

    Serial.println("Connected to server");
    return true;
}

void draw_jpg(const char* filename, int xpos, int ypos) {
    if (!LittleFS.exists(filename)) {
        Serial.printf("File %s does not exist\n", filename);
        return;
    }

    fs::File jpgFile = LittleFS.open(filename, "r");
    if (!jpgFile) {
        Serial.printf("Failed to open file %s\n", filename);
        return;
    }
    
    uint16_t *pImage;
    int16_t mcu_x, mcu_y;
    uint16_t mcu_w, mcu_h;

    int ret = JpegDec.decodeFsFile(jpgFile);
    if (ret != 1) {
        Serial.println("Failed to decode JPEG");
        return;
    }

    while (JpegDec.read()) {
        mcu_x = JpegDec.MCUx * JpegDec.MCUWidth + xpos;
        mcu_y = JpegDec.MCUy * JpegDec.MCUHeight + ypos;
        mcu_w = JpegDec.MCUWidth;
        mcu_h = JpegDec.MCUHeight;

        // Clip MCU block to screen bounds
        if (mcu_x + mcu_w >= tft.width()) mcu_w = tft.width() - mcu_x;
        if (mcu_y + mcu_h >= tft.height()) mcu_h = tft.height() - mcu_y;

        pImage = JpegDec.pImage;

        tft.drawRGBBitmap(mcu_x, mcu_y, pImage, NULL, mcu_w, mcu_h);
    }
    

    jpgFile.close();
}


void draw_bootloading_sequence() {
    tft.fillScreen(ILI9341_BLACK);
    delay(1000);
    draw_jpg("/logo.jpg", 0, 0);
    delay(1000);
    tft.fillScreen(ILI9341_BLACK);
    delay(500);
    tft.println("Welcome to the");
    tft.println("IoT Screen");
    delay(1000);
    tft.fillScreen(ILI9341_BLACK);
}

void print_fs_files()
{
    Serial.println("Printing files in the file system");
    fs::Dir dir = LittleFS.openDir("/");
    // print all files in the file system
   
    

    while (dir.next())
    {
        Serial.print(dir.fileName());
        Serial.print(" - ");
        Serial.println(dir.fileSize());
    }
}


void setup()
{
    Serial.begin(9600);

    if (!LittleFS.begin()) {
        Serial.println("Failed to mount file system");
        return;
    }

    print_fs_files();

    Serial.println("setup oled");

    tft.begin();
    tft.setRotation(3);
    draw_bootloading_sequence();

 

     init_wifi();
     connect_to_tcp();

      delay(2000);
}

void try_to_reconnect()
{
    if (!client.connected())
    {
        Serial.println("Reconnecting...");
        if (connect_to_tcp())
        {
            Serial.println("Reconnected");
        }
        else
        {
            Serial.println("Reconnection failed");
        }
        delay(1000);
    }
}

void process_message()
{
    uint8_t buffer[1056];
    size_t bytes_read = client.read(buffer, sizeof(buffer));

    if (bytes_read == 0)
    {
        Serial.println("No data received; can't process message");
        return;
    }

    Serial.printf("Received %d bytes:\n", bytes_read);
    for (size_t i = 0; i < bytes_read; i++)
    {
        Serial.printf("%02x ", buffer[i]);
    }
    Serial.println();

    auto message = ScreenIoT::GetMessage(buffer);
    auto app = message->app()->c_str();
    auto payload = message->payload()->c_str();

    Serial.printf("App: %s\n", app);
    if (app == "PING")
    {
        Serial.println("Received PING message");
        return;
    }

    Serial.printf("App: %s\n", app);
    tft.fillScreen(ILI9341_BLACK);
    tft.setCursor(0, 0);
    tft.setTextColor(ILI9341_WHITE);
    tft.setTextSize(2);
    tft.println(app);
    tft.println(payload);
    Serial.printf("Payload: %s\n", payload);
}



void loop()
{

    for (uint8_t rotation = 0; rotation < 4; rotation++)
    {
        tft.setRotation(rotation);
        delay(1000);
    }

    try_to_reconnect();

    tft.drawCircle(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 32, ILI9341_GREEN);

    if (client.available()) {
      Serial.println("Processing message");
      process_message();
    }
    

    delay(500);
}