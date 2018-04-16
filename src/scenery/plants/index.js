import Phaser from 'phaser'

export default class Plant extends Phaser.GameObjects.Image {
  static key = 'plant-pot'
  static preload(scene) {
    scene.load.image(Plant.key, require('./plant-pot.png'))
  }

  constructor({ scene, x, y }) {
    super(scene, x, y, Plant.key, 0)

    this.type = 'scenery'

    this.scene.add.existing(this)
  }
}
