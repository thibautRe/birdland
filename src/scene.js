import Phaser from 'phaser'

import Bird from './bird'

export default class Scene extends Phaser.Scene {
  constructor() {
    super({
      key: 'MainScene'
    })
  }

  preload() {
    Bird.preload(this)
  }

  create() {
    this.bird = new Bird({
      scene: this,
      x: 100,
      y: 100,
    })

    this.cursors = this.input.keyboard.createCursorKeys();
  }

  update() {
    const { cursors } = this

    this.bird.update({
      cursors,
    })
  }
}
