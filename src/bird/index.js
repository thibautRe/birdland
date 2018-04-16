import Phaser from 'phaser'

export default class Bird extends Phaser.GameObjects.Sprite {
  static key = 'bird'
  static preload(scene) {
    scene.load.spritesheet(Bird.key, require('./bird.png'), {
      frameWidth: 32,
      frameHeight: 22,
    })
  }

  constructor({ scene, x, y }) {
    super(scene, x, y, Bird.key, 0)

    this.type = 'bird'
    this.scene.physics.world.enable(this)
    this.scene.add.existing(this)
    this.scene.anims.create({
      key: 'down',
      frames: this.scene.anims.generateFrameNumbers(Bird.key, {
        start: 0,
        end: 2,
      }),
      frameRate: 10,
      repeat: -1,
    })
  }

  update({ cursors }) {
    if (cursors.down.isDown) {
      this.body.setVelocityY(100)
      this.anims.play('down', true)
      this.flipY = false
    } else if (cursors.up.isDown) {
      this.body.setVelocityY(-100)
      this.anims.play('down', true)
      this.flipY = true
    } else {
      this.body.setVelocityY(0)
      this.anims.stop()
    }
  }
}
