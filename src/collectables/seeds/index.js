import Phaser from 'phaser'

export default class Seeds extends Phaser.GameObjects.Sprite {
  static key = 'seeds'
  static preload(scene) {
    scene.load.image(Seeds.key, require('./seeds.png'))
  }

  constructor({ scene, x, y }) {
    super(scene, x, y, Seeds.key)

    this.type = 'collectable'

    this.scene.add.existing(this)
  }
}
