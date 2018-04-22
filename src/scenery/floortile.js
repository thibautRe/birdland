import Phaser from 'phaser'
import getRandomColor from '../utils/getRandomColor'

export default class FloorTile extends Phaser.GameObjects.Graphics {
  constructor({ x, y, scene, width, height }) {
    super(scene)

    this.rectangle = new Phaser.Geom.Rectangle(x, y, width, height)

    this.fillStyle(getRandomColor([65, 75], [170, 190], [80, 90]))
    this.fillRectShape(this.rectangle)

    this.scene.add.existing(this)
  }
}
