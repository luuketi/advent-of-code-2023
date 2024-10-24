#![allow(dead_code)]

pub const INPUT1 : &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

pub const INPUT2 : &str = "Game 1: 1 blue, 8 green; 14 green, 15 blue; 3 green, 9 blue; 8 green, 8 blue, 1 red; 1 red, 9 green, 10 blue
Game 2: 3 blue, 1 green, 2 red; 2 red, 2 green, 5 blue; 3 green, 10 blue; 8 red, 1 blue; 3 red, 1 green, 5 blue; 1 blue, 5 red, 3 green
Game 3: 4 green, 1 blue; 6 blue, 5 green, 1 red; 11 green, 10 blue
Game 4: 12 blue, 12 green, 3 red; 15 blue, 1 green, 10 red; 8 blue, 3 red, 2 green; 14 red, 8 blue
Game 5: 7 blue, 8 red, 5 green; 15 blue, 16 red, 14 green; 3 blue, 14 red, 10 green
Game 6: 4 blue, 13 red; 1 green, 13 blue, 11 red; 4 red, 19 blue; 18 blue, 10 red, 1 green
Game 7: 8 green, 3 blue, 3 red; 2 red, 7 green, 10 blue; 6 green, 11 red, 3 blue
Game 8: 10 red, 6 green, 1 blue; 15 green, 10 red, 3 blue; 8 red, 10 green, 5 blue
Game 9: 2 green, 8 blue, 1 red; 6 blue, 10 red; 13 blue, 12 red, 7 green
Game 10: 2 blue, 8 red, 10 green; 1 green, 2 blue; 1 red, 1 green; 7 red, 2 blue, 1 green
Game 11: 8 green, 1 blue; 6 green; 2 green, 1 blue; 2 blue, 11 green; 1 red, 12 green
Game 12: 3 red, 2 green, 15 blue; 1 blue, 1 green, 4 red; 1 green, 12 blue, 3 red; 1 red, 10 blue; 3 red, 2 green, 14 blue; 3 red, 13 blue
Game 13: 7 blue, 5 red; 7 red, 3 green, 9 blue; 9 green, 7 blue, 7 red; 6 blue, 8 red; 11 red; 3 green, 7 blue, 8 red
Game 14: 4 blue, 6 green, 7 red; 8 red, 4 green, 11 blue; 3 green, 9 red, 13 blue
Game 15: 3 green, 1 blue, 5 red; 2 red; 1 red, 4 green
Game 16: 1 green, 7 blue; 3 red, 5 blue; 1 green, 5 blue; 5 blue, 1 green; 1 green, 1 red, 13 blue
Game 17: 4 blue, 2 red, 4 green; 1 blue, 7 red, 4 green; 4 red, 4 green, 10 blue; 1 blue, 4 red, 14 green
Game 18: 7 blue, 5 green; 4 blue, 3 green; 1 red, 6 green, 7 blue
Game 19: 10 blue, 3 red, 6 green; 3 blue, 4 red, 17 green; 19 green, 3 red, 3 blue; 19 green, 3 blue; 4 red, 7 green, 7 blue; 10 blue, 13 green, 1 red
Game 20: 3 blue, 6 red, 1 green; 6 green, 7 red, 18 blue; 1 green, 5 red, 14 blue; 1 green, 12 blue, 8 red
Game 21: 16 blue, 7 green, 13 red; 11 red, 7 blue, 5 green; 4 green, 3 blue
Game 22: 14 blue, 6 red, 1 green; 9 red, 1 green, 11 blue; 3 red, 13 blue; 6 red, 10 blue; 13 red, 1 green, 2 blue
Game 23: 17 red, 1 blue, 13 green; 19 green, 1 blue, 3 red; 7 red, 19 green; 16 red, 10 green; 16 red, 12 green, 1 blue
Game 24: 1 green, 2 blue; 10 green, 4 blue; 8 blue, 11 green, 14 red
Game 25: 9 blue, 10 red; 2 red, 7 green, 5 blue; 4 green, 10 red, 5 blue; 6 red, 6 blue; 12 blue, 4 green
Game 26: 9 red, 2 blue, 5 green; 3 red, 4 green, 1 blue; 5 red, 2 blue, 13 green
Game 27: 1 green, 14 blue, 2 red; 9 red, 7 blue, 7 green; 9 blue, 10 red, 7 green; 1 blue, 5 red, 3 green; 1 blue, 4 red; 9 red, 1 green
Game 28: 11 red, 13 blue, 12 green; 8 blue, 4 green, 6 red; 2 blue, 9 green
Game 29: 5 green, 16 red, 1 blue; 6 blue, 3 green, 2 red; 1 green, 7 blue, 9 red
Game 30: 1 green, 2 blue, 2 red; 7 blue, 5 red; 2 red, 1 blue; 6 red, 1 green, 6 blue
Game 31: 5 green, 7 blue, 14 red; 19 red, 9 green, 9 blue; 4 green, 8 red, 12 blue; 20 red, 12 green; 10 red, 3 green, 6 blue; 5 blue, 17 red, 8 green
Game 32: 6 blue, 6 red, 8 green; 6 blue, 3 green, 7 red; 4 red, 6 green, 4 blue; 5 green, 3 blue, 5 red; 8 blue, 6 red, 5 green
Game 33: 5 red, 15 green, 3 blue; 4 green, 8 red; 6 blue, 17 green, 2 red
Game 34: 12 blue, 1 green; 3 red, 14 blue, 1 green; 1 green, 16 blue, 3 red
Game 35: 2 red, 16 blue; 17 blue, 5 green; 10 blue, 3 green; 1 blue, 2 green; 4 blue, 4 green
Game 36: 7 blue, 8 red, 4 green; 3 red, 13 green, 14 blue; 17 green, 2 blue, 8 red; 2 red, 13 blue, 2 green; 12 blue, 1 red, 9 green; 12 green, 10 blue
Game 37: 5 green; 3 green, 14 red; 2 red, 1 blue; 11 green, 1 blue; 8 green, 18 red, 1 blue; 1 blue, 16 red
Game 38: 5 red; 9 green, 11 blue, 7 red; 2 blue, 2 green, 1 red; 3 blue, 7 red; 5 red, 8 blue
Game 39: 1 blue, 7 green, 6 red; 18 green, 2 red; 1 blue, 19 green, 6 red; 2 green, 3 blue, 9 red; 14 green, 4 red, 3 blue; 16 green, 4 red, 1 blue
Game 40: 14 red, 2 green, 2 blue; 2 blue, 2 red, 4 green; 8 red, 2 blue; 1 green, 17 red; 10 red; 2 green, 3 red
Game 41: 11 green, 1 red, 1 blue; 2 blue, 18 green, 3 red; 2 blue, 2 green, 8 red; 7 red, 1 blue, 17 green; 14 green, 2 blue; 2 blue, 14 red, 18 green
Game 42: 2 red, 3 green, 15 blue; 3 green, 18 blue; 9 blue, 2 red
Game 43: 4 green, 2 blue; 10 blue, 2 red, 14 green; 13 green, 1 red, 11 blue; 9 green, 10 blue; 14 green, 8 red, 8 blue
Game 44: 10 red; 3 red, 8 blue, 1 green; 10 red, 2 blue, 1 green; 5 green, 7 blue, 17 red; 3 green, 18 red, 6 blue; 3 green, 11 blue, 3 red
Game 45: 10 blue, 5 green; 14 blue, 5 green; 10 green, 14 blue, 1 red; 1 red, 5 green, 18 blue; 5 green, 5 blue, 1 red; 17 blue, 12 green, 1 red
Game 46: 2 blue, 2 green; 1 green, 1 blue; 2 blue, 3 green; 1 red; 2 green
Game 47: 6 blue, 1 red, 12 green; 2 red, 3 green, 4 blue; 1 blue, 13 green, 6 red; 12 green, 4 blue, 5 red
Game 48: 4 green, 5 red; 19 green, 1 blue, 11 red; 4 red, 8 green; 10 red, 1 blue, 16 green
Game 49: 12 red, 2 blue; 6 green; 1 green, 9 red
Game 50: 1 green, 1 blue, 17 red; 1 blue, 14 red, 1 green; 2 blue, 6 red
Game 51: 12 green, 9 blue, 1 red; 6 green, 2 blue, 1 red; 14 green, 5 blue; 1 green, 2 red, 12 blue; 4 green, 2 red, 8 blue; 1 green, 5 blue
Game 52: 1 green, 7 blue, 3 red; 6 blue, 5 red, 5 green; 7 green; 5 blue, 1 green, 6 red
Game 53: 3 blue, 1 red; 1 blue, 3 green; 2 green, 5 blue; 4 blue, 3 green; 1 green; 2 green, 5 blue, 1 red
Game 54: 13 blue, 3 red; 17 blue, 1 green, 8 red; 2 green, 2 red, 11 blue; 2 green, 1 red, 4 blue; 8 red
Game 55: 1 green; 3 blue, 5 red; 1 blue, 1 green, 4 red; 1 red, 10 blue; 4 red, 17 blue
Game 56: 4 blue, 12 green, 12 red; 6 green, 10 blue, 2 red; 8 green, 11 red, 3 blue; 6 green, 10 red, 10 blue; 7 red, 5 green
Game 57: 2 green, 8 blue, 3 red; 17 blue, 1 green, 9 red; 4 red, 7 blue
Game 58: 4 blue, 1 red; 2 blue, 1 green; 2 red, 5 green, 4 blue; 7 green, 5 blue, 2 red
Game 59: 4 green, 5 red; 4 green, 7 red, 1 blue; 15 red, 8 green
Game 60: 6 green, 3 red; 4 red, 6 green, 4 blue; 5 green, 4 blue, 1 red; 3 red, 4 blue, 2 green; 7 red, 3 blue; 1 green, 1 blue
Game 61: 7 red, 10 blue, 7 green; 11 blue, 9 green, 1 red; 11 red, 1 green, 3 blue; 3 green, 13 red, 2 blue; 7 green, 2 blue, 1 red
Game 62: 10 green, 9 red; 6 blue, 10 red, 11 green; 7 red, 2 blue, 2 green
Game 63: 10 green, 2 blue; 1 red; 9 green; 2 blue, 8 green; 1 blue, 1 green, 1 red
Game 64: 2 green; 8 red; 4 green, 5 red, 2 blue
Game 65: 2 red, 16 blue; 5 green, 11 blue, 3 red; 15 blue, 4 green, 3 red
Game 66: 2 green, 3 blue; 1 red, 3 blue; 1 green, 4 blue; 1 blue, 2 green, 1 red; 2 blue; 1 red, 3 green, 5 blue
Game 67: 2 blue, 11 red, 8 green; 13 blue, 19 red, 13 green; 3 red, 3 green, 3 blue; 2 red, 5 blue, 16 green; 13 red, 4 blue, 5 green
Game 68: 1 blue, 9 red, 7 green; 3 green, 9 red, 4 blue; 7 green, 5 blue; 3 green, 9 red, 3 blue; 5 red, 1 blue, 8 green; 8 green, 1 blue, 4 red
Game 69: 10 green, 1 red, 2 blue; 1 red, 11 green; 1 blue, 1 green; 8 green; 11 green, 1 red; 1 green, 3 blue
Game 70: 8 red, 15 blue, 18 green; 14 blue, 9 red, 8 green; 6 blue, 2 red, 2 green; 1 green, 4 blue, 9 red; 3 blue, 15 green, 10 red
Game 71: 10 red, 10 green, 6 blue; 3 green, 6 blue, 13 red; 7 green, 6 red, 12 blue
Game 72: 12 red, 5 blue; 13 red, 6 green, 11 blue; 11 green, 7 red, 11 blue
Game 73: 8 blue, 1 red, 3 green; 1 blue; 6 blue, 4 green
Game 74: 1 red, 3 blue, 8 green; 6 green, 3 blue, 1 red; 18 green, 1 red; 3 blue, 1 red, 14 green; 5 green, 1 blue; 3 blue, 1 green
Game 75: 4 blue, 1 red, 3 green; 6 blue; 11 green, 6 blue, 3 red; 10 green, 4 red; 2 blue, 2 red, 6 green
Game 76: 7 blue; 15 blue, 2 red, 1 green; 2 blue, 5 red; 2 red, 15 blue; 4 red, 15 blue; 9 blue, 5 red
Game 77: 12 blue, 8 green, 15 red; 12 blue, 19 red, 16 green; 6 blue, 5 green, 16 red
Game 78: 2 red, 7 blue, 14 green; 1 red, 3 green, 1 blue; 4 blue, 8 green, 10 red
Game 79: 3 red, 5 green; 2 blue, 1 red, 18 green; 4 red, 15 green, 2 blue; 18 green, 2 blue, 7 red; 7 green, 6 red
Game 80: 8 green, 5 red, 9 blue; 14 blue, 13 red, 6 green; 14 blue, 7 red, 4 green; 3 blue, 16 red, 4 green; 5 green, 13 blue, 2 red; 16 blue, 2 green, 5 red
Game 81: 10 green, 10 red, 4 blue; 13 red, 2 blue; 8 green, 10 red, 7 blue
Game 82: 1 blue, 4 red, 7 green; 3 green, 2 blue, 3 red; 1 blue, 3 red, 3 green; 6 red, 4 green, 5 blue; 3 blue, 6 green
Game 83: 3 red, 1 blue, 2 green; 6 blue, 2 green, 2 red; 6 green, 1 blue; 11 blue, 4 red, 10 green
Game 84: 9 red, 12 green, 1 blue; 7 red, 5 green, 3 blue; 3 green, 8 blue, 11 red; 1 red, 3 blue, 10 green
Game 85: 2 blue, 2 red; 3 blue, 3 red; 2 green, 2 red; 2 red, 4 green
Game 86: 2 green, 5 blue, 9 red; 4 red, 2 green, 5 blue; 6 red, 2 green, 6 blue; 4 red, 4 blue, 1 green
Game 87: 17 red, 3 blue, 2 green; 5 red, 1 green, 5 blue; 4 blue, 3 green, 7 red
Game 88: 7 green, 1 blue, 1 red; 1 blue, 1 red, 10 green; 1 blue, 2 green; 5 green; 7 green, 1 blue; 1 red, 3 green
Game 89: 6 red, 11 blue, 3 green; 3 blue, 4 green, 3 red; 4 blue, 17 red, 4 green; 17 blue, 3 green, 4 red; 3 red, 9 blue, 4 green; 6 red, 9 blue, 9 green
Game 90: 9 red, 4 green, 1 blue; 6 green, 5 red, 11 blue; 3 blue, 8 red
Game 91: 3 red, 7 blue, 1 green; 11 blue, 3 green, 4 red; 1 red, 8 blue, 3 green; 10 red, 4 blue, 3 green
Game 92: 2 blue, 1 red; 7 red, 1 blue; 4 green, 11 red, 5 blue
Game 93: 13 red, 11 green, 10 blue; 3 blue, 10 green; 3 blue, 7 red, 3 green
Game 94: 9 green, 8 blue, 4 red; 7 blue, 9 green, 5 red; 16 green, 5 red; 4 blue
Game 95: 9 green; 1 blue, 16 green; 2 green, 2 red; 1 green, 1 blue; 6 green; 7 green, 2 red
Game 96: 6 green, 8 red, 5 blue; 2 blue, 14 green, 17 red; 2 blue, 15 green, 16 red; 16 green, 1 red; 16 red, 2 green, 1 blue; 18 red, 13 green, 6 blue
Game 97: 9 green, 8 red, 11 blue; 11 green, 13 red, 4 blue; 7 blue, 1 green, 6 red; 1 blue, 12 red, 7 green
Game 98: 1 green, 12 blue, 4 red; 8 blue, 5 red, 1 green; 1 green, 8 blue, 7 red; 1 green, 5 red, 6 blue; 3 blue, 1 green, 1 red; 4 blue, 1 green, 4 red
Game 99: 1 blue, 2 green, 2 red; 2 red, 8 green; 14 green, 1 blue; 1 red, 2 green; 1 blue, 1 green, 2 red; 6 green, 2 red
Game 100: 6 green, 15 red, 12 blue; 9 red; 16 red; 17 red, 3 blue, 7 green";
