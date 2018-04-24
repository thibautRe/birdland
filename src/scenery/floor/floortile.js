import Phaser from 'phaser'

export default class FloorTile extends Phaser.GameObjects.Graphics {
  color = Phaser.Display.Color.GetColor(255, 200, 200)

  constructor({ x, y, scene, width, height }) {
    super(scene)

    this.rectangle = new Phaser.Geom.Rectangle(x, y, width, height)
    this.scene.add.existing(this)
  }

  fill() {
    this.fillStyle(this.color)
    this.fillRectShape(this.rectangle)
  }
}
