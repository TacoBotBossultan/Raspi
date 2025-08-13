/// DrivESP
#include <Wire.h>
#include <RoboClaw.h>
#include <math.h>

enum {
  PINPOINT_I2C_ADDR = 0x31,
  ROBOCLAWFront_ADDRESS = 0x80,
  ROBOCLAWRear_ADDRESS = 0x81,
  IMU_Read_DeviceID = 0x01,
  IMU_Read_DeviceVersion = 0x02,
  IMU_Read_DeviceStatus = 0x03,
  IMU_Write_DeviceControl = 0x04,
  IMU_Read_LoopTime = 0x05,
  IMU_Read_XEncoderVal = 0x06,
  IMU_Read_YEncoderVal = 0x07,
  IMU_XPos = 0x08,
  IMU_YPos = 0x09,
  IMU_Head = 0x0A,
  IMU_TicksPerMM = 0x0E,
  IMU_XOffset = 0x0F,
  IMU_YOffset = 0x10,
  Have_Pos = 0x33,
  Give_Me_Pos = 0x34,
  Set_Speed = 0x35,
  Set_Pos = 0x45,
  IMU_BulkRead = 0x12,
  Speed_Max = 2000,
  Accel = 2000,
  deadZoneMin = 99,
  deadZoneMax = 101
};

HardwareSerial SerialCOM(2);
RoboClaw roboclaw(&SerialCOM, 10000);

float xPos, yPos, head;
char raspunsPoz[16];
char comanda[16];  
int v1, v2, v3, v4;
float newX,newY;
uint8_t rawData[40];

void setup() {

  Serial.begin(115200);
  SerialCOM.begin(115200, SERIAL_8N1, 16, 17);
  Wire.begin();
  delay(1000);

  roboclaw.SpeedAccelM1M2(ROBOCLAWFront_ADDRESS, Accel, 0, 0);
  roboclaw.SpeedAccelM1M2(ROBOCLAWRear_ADDRESS, Accel, 0, 0);

  writeFloatToRegister(IMU_TicksPerMM, 19.894);
  writeFloatToRegister(IMU_XOffset, 252);
  writeFloatToRegister(IMU_YOffset, -264);

  writeFloatToRegister(IMU_XPos, 0);
  writeFloatToRegister(IMU_YPos, 0);
  writeFloatToRegister(IMU_Head, 0);

  raspunsPoz[0] = Have_Pos;
}

void loop() {
  if(Serial.available()) {  

    delayMicroseconds(500);
    for (int i = 0; i < 16; i++) {
        comanda[i] = Serial.read();
        delayMicroseconds(500);
      }

    switch(comanda[0]) {
      case Give_Me_Pos: {
        getPosition();
        Serial.write(raspunsPoz, 16);
        break;
      }

      case Set_Speed: {
        v1 = (int)comanda[1];
        v2 = (int)comanda[2];
        v3 = (int)comanda[3];
        v4 = (int)comanda[4];

        if (v1 >= deadZoneMin && v1 <= deadZoneMax) v1 = 0;
        else v1 = map(v1, 1, 200, -Speed_Max, Speed_Max);

        if (v2 >= deadZoneMin && v2 <= deadZoneMax) v2 = 0;
        else v2 = map(v2, 1, 200, -Speed_Max, Speed_Max);

        if (v3 >= deadZoneMin && v3 <= deadZoneMax) v3 = 0;
        else v3 = map(v3, 1, 200, -Speed_Max, Speed_Max);
        
        if (v4 >= deadZoneMin && v4 <= deadZoneMax) v4 = 0;
        else v4 = map(v4, 1, 200, -Speed_Max, Speed_Max);
        
        roboclaw.SpeedAccelM1M2(ROBOCLAWFront_ADDRESS, Accel, v1, v2);
        roboclaw.SpeedAccelM1M2(ROBOCLAWRear_ADDRESS, Accel, v4, v3);

        break;
      }

      case Set_Pos: {
        memcpy(&newX, &comanda[1], sizeof(float));
        memcpy(&newY, &comanda[5], sizeof(float));

        uint16_t headInt = comanda[9] | (comanda[10] << 8);
        float newHead = (float)headInt;

        writeFloatToRegister(IMU_XPos, newX);
        writeFloatToRegister(IMU_YPos, newY);
        writeFloatToRegister(IMU_Head, newHead);
        break;
      }

      default: {
        Serial.print("Unknown command: ");
        Serial.println(comanda);
        break;
      }
    }
  }
}

void getPosition() {

  readBulkRegister(IMU_BulkRead, rawData, 40);

  memcpy(&xPos, &rawData[16], sizeof(float));
  memcpy(&yPos, &rawData[20], sizeof(float));
  memcpy(&head, &rawData[24], sizeof(float));

  head = head * 57.29;
  if(head < 0) head = 360.0f - fmod(fabs(head), 360.0f);
  else head = fmod(head, 360.0f);
  uint16_t head_int = (uint16_t)roundf(head);
  

  memcpy(&raspunsPoz[1], &xPos, sizeof(float));       
  memcpy(&raspunsPoz[5], &yPos, sizeof(float));       
  memcpy(&raspunsPoz[9], &head_int, sizeof(uint16_t));   
  memset(&raspunsPoz[13], 0x30, 5); 

}


bool writeFloatToRegister(uint8_t registerAddress, float value) {

  Wire.beginTransmission(PINPOINT_I2C_ADDR);
  Wire.write(registerAddress);

  uint8_t *p = (uint8_t *)&value;
  for (int i = 0; i < 4; i++) {
    Wire.write(p[i]);
  }

  uint8_t result = Wire.endTransmission();
  return (result == 0);  
}

bool readBulkRegister(uint8_t reg, uint8_t *buffer, size_t len) {
  Wire.beginTransmission(PINPOINT_I2C_ADDR);
  Wire.write(reg);

  if (Wire.endTransmission(false) != 0) {
    Serial.print("Nu pot trimite comanda la reg: ");
    Serial.println(reg);
    return false;
  }
  
  uint8_t bytesRead = Wire.requestFrom(PINPOINT_I2C_ADDR, (uint8_t)len);
  if (bytesRead != len) {
    return false;
  }

  for (size_t i = 0; i < len; i++) {
    if (Wire.available()) {
      buffer[i] = Wire.read();
    } else {
      Serial.println("Buld read fail");
      return false;
    }
  }
  return true;
}


