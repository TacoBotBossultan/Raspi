/// UtilitiESP
#include <Servo.h>

enum {
  No = 0x31,
  Yes = 0x32,
  onLED = 0x36,
  offLED = 0x37,
  BtnPressed = 0x38,
  ReachedBand = 0x39,
  PushRack = 0x41,
  IsItIn = 0x42,
  PullRack = 0x43,
  IsItOut = 0x44,
  Adu_O_Bere = 0x69,
  ROBOCLAW_ADDRESS = 0x80,
  VitezaFata = 1800,
  VitezaSpate = 1200,
  VitezaZero = 1500,
  VitezaMica = 1600,
  Rack_Acasa = 0,
  Rack_Trebuie_Impins = 1,
  Rack_In_Impingere = 2,
  Rack_Impins = 3,
  Rack_Trebuie_Retras = 4,
  Rack_In_Retragere = 5,
  Rack_Inapoi_La_Poz_Initiala = 6,
  BR_Acasa = 10,
  BR_In_Retragere = 11,
  BR_Retras = 12,
  BR_In_Impingere = 13,
};

bool aFostLow = false;
int CameraLEDpin = 5;
int Bumper1pin = A1;
int Bumper2pin = A2;
int LaneMarkerPin = A3;
int MotorPin = 9;
int EndStopPin = A4;
int EncoderPin = A5;
int PosRackIn = 51;
int PosRackOut = 0;
int stare_rack = 0;
int stare_BR = 10;
int encodor = 1;
char comanda[16];
char raspunsOk[16];
char raspunsNok[16];

Servo LED;
Servo motor;
Servo motorBR;

/// Berevolver (BR)

const int motorBerevolverPin = 11;
const int limitSwtichInBR = 12;
const int limitSwitchOutBR = 13;
int nr_cicluri = 0;

void setup() {

  Serial.begin(115200);
  pinMode(CameraLEDpin, OUTPUT);
  pinMode(MotorPin, OUTPUT);
  pinMode(Bumper1pin, INPUT_PULLUP);
  pinMode(Bumper2pin, INPUT_PULLUP);
  pinMode(LaneMarkerPin, INPUT_PULLUP);
  pinMode(EndStopPin, INPUT_PULLUP);
  pinMode(EncoderPin, INPUT_PULLUP);
  pinMode(limitSwitchOutBR, INPUT_PULLUP);
  pinMode(limitSwtichInBR, INPUT_PULLUP);

  delay(1000);

  LED.attach(CameraLEDpin);
  LED.writeMicroseconds(0);

  motor.attach(MotorPin, VitezaSpate, VitezaFata);
  motor.writeMicroseconds(VitezaZero);

  motorBR.attach(motorBerevolverPin, VitezaSpate, VitezaFata);
  motorBR.writeMicroseconds(VitezaZero);

  homing();
  homingBR();

  raspunsOk[0] = Yes;
  memset(&raspunsOk[1], 0x30, 15);  //2
  raspunsNok[0] = No;
  memset(&raspunsNok[1], 0x30, 15);  //1
}

void loop() {
  if (Serial.available()) {

    delayMicroseconds(500);
    for (int i = 0; i < 16; i++) {
      comanda[i] = Serial.read();
      delayMicroseconds(500);
    }

    switch (comanda[0]) {

      case onLED:
        {
          digitalWrite(13, HIGH);
          LED.write(180);
          break;
        }

      case offLED:
        {
          digitalWrite(13, LOW);
          LED.write(0);
          break;
        }

      case BtnPressed:
        {
          if (digitalRead(Bumper1pin) == LOW && digitalRead(Bumper2pin) == LOW) Serial.write(raspunsOk, 16);
          else Serial.write(raspunsNok, 16);
          break;
        }

      case ReachedBand:
        {
          if (digitalRead(LaneMarkerPin) == HIGH) Serial.write(raspunsOk, 16);
          else Serial.write(raspunsNok, 16);
          break;
        }

      case PushRack:
        {
          if (stare_rack == Rack_Acasa) stare_rack = Rack_Trebuie_Impins;
          break;
        }

      case IsItIn:
        {
          if (stare_rack == Rack_Impins) Serial.write(raspunsOk, 16);
          else Serial.write(raspunsNok, 16);
          Serial.println(encodor);
          break;
        }

      case PullRack:
        {
          if (stare_rack == Rack_Impins || stare_rack == Rack_In_Impingere) {
            if (stare_rack == Rack_In_Impingere) {
              decelerate(VitezaFata);
            }
            stare_rack = Rack_Trebuie_Retras;
          }
          break;
        }

      case IsItOut:
        {
          if (stare_rack == Rack_Acasa) Serial.write(raspunsOk, 16);
          else Serial.write(raspunsNok, 16);
          break;
        }

      case Adu_O_Bere:
        {
          if (stare_BR == BR_Acasa) {
            accelerateBR(VitezaSpate);
            stare_BR = BR_In_Retragere;
          }
        }

      default:
        {
          Serial.print("Unknown command: ");
          Serial.println(comanda);
        }
    }
  }

  operare_rack();
  operare_BR();
  delay(50);
}

void operare_rack() {
  switch (stare_rack) {

    case Rack_Acasa:
      {
        encodor = 1;
        break;
      }

    case Rack_Trebuie_Impins:
      {
        accelerate(VitezaFata);
        stare_rack = Rack_In_Impingere;
        break;
      }

    case Rack_In_Impingere:
      {
        if (digitalRead(EncoderPin) == HIGH && aFostLow) {
          encodor++;
          aFostLow = false;
        } else if (digitalRead(EncoderPin) == LOW) {
          aFostLow = true;
        }

        if (encodor >= PosRackIn) {
          decelerate(VitezaFata);
          stare_rack = Rack_Impins;
        }
        break;
      }

    case Rack_Trebuie_Retras:
      {
        accelerate(VitezaSpate);
        stare_rack = Rack_In_Retragere;
        break;
      }

    case Rack_In_Retragere:
      {
        if (digitalRead(EncoderPin) == HIGH && aFostLow) {
          encodor--;
          aFostLow = false;
        } else if (digitalRead(EncoderPin) == LOW) {
          aFostLow = true;
        }

        if (digitalRead(EndStopPin) == HIGH) {
          decelerate(VitezaSpate);
          stare_rack = Rack_Inapoi_La_Poz_Initiala;
          accelerate(VitezaMica);
        }
        break;
      }

    case Rack_Inapoi_La_Poz_Initiala:
      {
        if (digitalRead(EncoderPin) == LOW) {
          motor.writeMicroseconds(VitezaZero);
          stare_rack = Rack_Acasa;
        }
        break;
      }
  }
}

void homing() {
  if (digitalRead(EndStopPin) == LOW) accelerate(VitezaSpate);
  while (1) {
    if (digitalRead(EndStopPin) == HIGH) {
      decelerate(VitezaSpate);
      break;
    }
    delay(20);
  }
  delay(500);

  accelerate(VitezaMica);
  while (1) {
    if (digitalRead(EncoderPin) == LOW) {
      motor.writeMicroseconds(VitezaZero);
      break;
    }
  }

  encodor = 1;
}

void accelerate(int targetVelocity) {
  if (targetVelocity >= VitezaZero) {
    int pas = (targetVelocity - VitezaZero) / 5;
    for (int viteza = VitezaZero; viteza < targetVelocity; viteza += pas) {
      motor.writeMicroseconds(viteza);
      delay(20);
    }
    motor.writeMicroseconds(targetVelocity);
    return;
  }

  int pas = (VitezaZero - targetVelocity) / 5;
  for (int viteza = VitezaZero; viteza > targetVelocity; viteza -= pas) {
    motor.writeMicroseconds(viteza);
    delay(20);
  }
  motor.writeMicroseconds(targetVelocity);
}

void decelerate(int currentVelocity) {
  if (currentVelocity >= VitezaZero) {
    int pas = (currentVelocity - VitezaZero) / 5;
    for (int viteza = currentVelocity; viteza > VitezaZero; viteza -= pas) {
      motor.writeMicroseconds(viteza);
      delay(20);
    }
    motor.writeMicroseconds(VitezaZero);
    return;
  }

  int pas = (VitezaZero - currentVelocity) / 5;
  for (int viteza = currentVelocity; viteza < VitezaZero; viteza += pas) {
    motor.writeMicroseconds(viteza);
    delay(20);
  }
  motor.writeMicroseconds(VitezaZero);
}

/// Berevolver homing si operare
void homingBR() {
  if (digitalRead(limitSwitchOutBR) == LOW) accelerateBR(VitezaFata);
  while (1) {
    if (digitalRead(limitSwitchOutBR) == HIGH) {
      decelerateBR(VitezaFata);
      break;
    }
    delay(20);
  }
  delay(500);
}

void operare_BR() {
  switch (stare_BR) {

    case BR_Acasa:
      break;

    case BR_In_Retragere:
      {
        if (digitalRead(limitSwtichInBR) == HIGH) {
          decelerateBR(VitezaSpate);
          stare_BR = BR_Retras;
        }
        break;
      }

    case BR_Retras:
      {
        if (nr_cicluri >= 100) {
          nr_cicluri = 0;
          accelerateBR(VitezaFata);
          stare_BR = BR_In_Impingere;
        } else nr_cicluri++;
      }

    case BR_In_Impingere:
      {
        if (digitalRead(limitSwitchOutBR) == HIGH) {
          decelerateBR(VitezaFata);
          stare_BR = BR_Acasa;
        }
        break;
      }
  }
}

void accelerateBR(int targetVelocity) {
  if (targetVelocity >= VitezaZero) {
    int pas = (targetVelocity - VitezaZero) / 5;
    for (int viteza = VitezaZero; viteza < targetVelocity; viteza += pas) {
      motorBR.writeMicroseconds(viteza);
      delay(20);
    }
    motorBR.writeMicroseconds(targetVelocity);
    return;
  }

  int pas = (VitezaZero - targetVelocity) / 5;
  for (int viteza = VitezaZero; viteza > targetVelocity; viteza -= pas) {
    motorBR.writeMicroseconds(viteza);
    delay(20);
  }
  motorBR.writeMicroseconds(targetVelocity);
}

void decelerateBR(int currentVelocity) {
  if (currentVelocity >= VitezaZero) {
    int pas = (currentVelocity - VitezaZero) / 5;
    for (int viteza = currentVelocity; viteza > VitezaZero; viteza -= pas) {
      motorBR.writeMicroseconds(viteza);
      delay(20);
    }
    motorBR.writeMicroseconds(VitezaZero);
    return;
  }

  int pas = (VitezaZero - currentVelocity) / 5;
  for (int viteza = currentVelocity; viteza < VitezaZero; viteza += pas) {
    motorBR.writeMicroseconds(viteza);
    delay(20);
  }
  motorBR.writeMicroseconds(VitezaZero);
}
