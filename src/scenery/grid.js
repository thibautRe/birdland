import Phaser from 'phaser'
import GrassTile from './floor/grass'
import SeaTile from './floor/sea'

export default class Grid extends Phaser.GameObjects.Group {
  constructor({ scene, height, width }) {
    super(scene)

    this.tileHeight = 30
    this.tileWidth = 30

    for (let i = 0; i < width - 1; i++) {
      for (let j = 0; j < height - 1; j++) {
        const Tile = this.getTileForGridPosition(i, j)
        this.add(
          new Tile({
            x: this.tileWidth * i,
            y: this.tileHeight * j,
            width: this.tileWidth,
            height: this.tileHeight,
            scene,
          }),
        )
      }
    }
  }

  getTileForGridPosition(i, j) {
    // Right now everything is just very very random
    return Math.round(Math.random() * 0.7) ? SeaTile : GrassTile
  }
}
