import Phaser from 'phaser'

export default class Bird extends Phaser.GameObjects.Sprite {
  static get key() { return 'bird' }
  static get sprite() { return require('./bird.png') }
  static get spriteDimensions() { return { frameWidth: 32, frameHeight: 22 }}

  static preload(scene) {
    scene.load.spritesheet(Bird.key, Bird.sprite, Bird.spriteDimensions)
    scene.load.image('particle', 'http://labs.phaser.io/assets/particles/blue.png');
  }

  constructor({ scene, x, y  }) {
    super(scene, x, y, Bird.key, 0)

    const particles = this.scene.add.particles('particle')
    const emitter = particles.createEmitter({
      speed: 200,
      scale: { start: 1, end: 0 },
      blendMode: 'ADD',
    })

    this.type = 'bird'

    this.scene.physics.world.enable(this);
    this.scene.add.existing(this);

    emitter.startFollow(this);

    this.scene.anims.create({
      key: 'down',
      frames: this.scene.anims.generateFrameNumbers(Bird.key, { start: 0, end: 2 }),
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
