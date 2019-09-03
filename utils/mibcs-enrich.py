#!/usr/bin/env python
import sys, json;

sex = 1 # 1=male, 0=female
age = 43 # in earth years
height = 181 # in centimeters

# based on https://github.com/oliexdev/openScale/blob/master/android_app/app/src/main/java/com/health/openscale/core/bluetooth/lib/MiScaleLib.java

def getLBMCoefficient(weight, impedance):
    lbm = (height * 9.058 / 100.0) * height / 100.0
    lbm += weight * 0.32 + 12.226
    lbm -= impedance * 0.0068
    lbm -= age * 0.0542
    return lbm

def getBMI(weight):
    return weight / (((height * height) / 100.0) / 100.0)

def getBodyFat(weight, impedance):
    bodyFat = 0.0
    lbmSub = 0.8

    if sex == 0 and age <= 49:
        lbmSub = 9.25
    if sex == 0 and age > 49:
        lbmSub = 7.25

    lbmCoeff = getLBMCoefficient(weight, impedance)
    coeff = 1.0

    if sex == 1 and weight < 61:
        coeff = 0.98
    if sex == 0 and weight > 60:
        coeff = 0.96

        if height > 160:
            coeff *= 1.03
    if sex == 0 and weight < 50:
        coeff = 1.02

        if height > 160:
            coeff *= 1.03

    bodyFat = (1.0 - (((lbmCoeff - lbmSub) * coeff) / weight)) * 100

    if bodyFat > 63:
        bodyFat = 75

    return bodyFat

def getVisceralFat(weight):
    visceralFat = 0.0

    if sex == 0:
        if weight > 13 - (height * 0.5) * -1:
            subsubcalc = ((height * 1.45) + (height * 0.1158) * height) - 120
            subcalc = weight * 500 / subsubcalc
            visceralFat = (subcalc - 6.0) + (age * 0.07)
        else:
            subcal = 0.691 + (height * -0.0024) + (height * -0.0024)
            visceralFat = (((height * 0.027) - (subcalc * weight)) * 01.0) + (age * 0.07) - age
    else:
        if height < weight * 1.6:
            subcalc = ((height * 0.5) - (height * (height * 0.0826))) * -1.0
            visceralFat = ((weight * 305.0) / (subcalc + 48.0)) - 2.9 + (age * 0.15)
        else:
            subcalc = 0.765 + height * -0.0015
            visceralFat = (((height * 0.143) - (weight * subcalc)) * -1.0) + (age * 0.15) - 5.0
    return visceralFat

def getBoneMass(weight, impedance):
    base = 0

    if sex == 0:
        base = 0.245691014
    else:
        base = 0.18016894

    boneMass = (base - (getLBMCoefficient(weight, impedance) * 0.05158)) * -1.0

    if boneMass > 2.2:
        boneMass += 0.1
    else:
        boneMass -= 0.1

    if sex == 0 and boneMass > 5.1:
        boneMass = 8.0
    if sex == 1 and boneMass > 5.2:
        boneMass = 8.0

    return boneMass

def getMuscle(weight, impedance):
    muscleMass = weight - ((getBodyFat(weight, impedance) * 0.01) * weight) - getBoneMass(weight, impedance)

    if sex == 0 and muscleMass >= 84:
        muscleMass = 120
    if sex == 1 and muscleMass >= 93.5:
        muscleMass = 120

    return muscleMass

def getWater(weight, impedance):
    water = (100.0 - getBodyFat(weight, impedance)) * 0.7

    if water < 50:
        coeff = 1.02
    else:
        coeff = 0.98

    return coeff * water

for line in iter(sys.stdin.readline, ''):
    data = json.loads(line)

    impedance = data['impedance']
    weight    = data['weight']

    if impedance is not None:
        data['bmi'] = getBMI(weight)
        data['body_fat_pct'] = getBodyFat(weight, impedance)
        data["water_pct"] = getWater(weight, impedance)
        data["visceral_fat"] = getVisceralFat(weight)
        data["bone_mass_kg"] = getBoneMass(weight, impedance)
        data["muscle_kg"] = getMuscle(weight, impedance)

    print json.dumps(data)
