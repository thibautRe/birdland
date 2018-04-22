import Phaser from 'phaser'
import getRandomNumber from './getRandomNumber'

export default (...intervals) =>
  Phaser.Display.Color.GetColor(...intervals.map(getRandomNumber))
