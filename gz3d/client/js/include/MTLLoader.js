/**
 * Loads a Wavefront .mtl file specifying materials
 *
 * @author angelxuanchang
 */

THREE.MTLLoader = function (manager) {
  this.manager = manager !== undefined ? manager : THREE.DefaultLoadingManager;
};

THREE.MTLLoader.prototype = {
  constructor: THREE.MTLLoader,

  /**
   * Loads and parses a MTL asset from a URL.
   *
   * @param {String} url - URL to the MTL file.
   * @param {Function} [onLoad] - Callback invoked with the loaded object.
   * @param {Function} [onProgress] - Callback for download progress.
   * @param {Function} [onError] - Callback for download errors.
   *
   * @see setPath setTexturePath
   *
   * @note In order for relative texture references to resolve correctly
   * you must call setPath and/or setTexturePath explicitly prior to load.
   */
  load: function (url, onLoad, onProgress, onError) {
    const scope = this;

    const loader = new THREE.FileLoader(this.manager);
    loader.setPath(this.path);
    loader.load(
      url,
      function (text) {
        onLoad(scope.parse(text));
      },
      onProgress,
      onError
    );
  },

  /**
   * Set base path for resolving references.
   * If set this path will be prepended to each loaded and found reference.
   *
   * @see setTexturePath
   * @param {String} path
   *
   * @example
   *     mtlLoader.setPath( 'assets/obj/' );
   *     mtlLoader.load( 'my.mtl', ... );
   */
  setPath: function (path) {
    this.path = path;
  },

  /**
   * Set base path for resolving texture references.
   * If set this path will be prepended found texture reference.
   * If not set and setPath is, it will be used as texture base path.
   *
   * @see setPath
   * @param {String} path
   *
   * @example
   *     mtlLoader.setPath( 'assets/obj/' );
   *     mtlLoader.setTexturePath( 'assets/textures/' );
   *     mtlLoader.load( 'my.mtl', ... );
   */
  setTexturePath: function (path) {
    this.texturePath = path;
  },

  setBaseUrl: function (path) {
    console.warn(
      "THREE.MTLLoader: .setBaseUrl() is deprecated. Use .setTexturePath( path ) for texture path or .setPath( path ) for general base path instead."
    );

    this.setTexturePath(path);
  },

  setCrossOrigin: function (value) {
    this.crossOrigin = value;
  },

  setMaterialOptions: function (value) {
    this.materialOptions = value;
  },

  /**
   * Parses a MTL file.
   *
   * @param {String} text - Content of MTL file
   * @return {THREE.MTLLoader.MaterialCreator}
   *
   * @see setPath setTexturePath
   *
   * @note In order for relative texture references to resolve correctly
   * you must call setPath and/or setTexturePath explicitly prior to parse.
   */
  parse: function (text) {
    const lines = text.split("\n");
    let info = {};
    const delimiter_pattern = /\s+/;
    const materialsInfo = {};

    for (let i = 0; i < lines.length; i++) {
      let line = lines[i];
      line = line.trim();

      if (line.length === 0 || line.charAt(0) === "#") {
        // Blank line or comment ignore
        continue;
      }

      const pos = line.indexOf(" ");

      let key = pos >= 0 ? line.substring(0, pos) : line;
      key = key.toLowerCase();

      let value = pos >= 0 ? line.substring(pos + 1) : "";
      value = value.trim();

      if (key === "newmtl") {
        // New material

        info = { name: value };
        materialsInfo[value] = info;
      } else if (info) {
        if (key === "ka" || key === "kd" || key === "ks") {
          const ss = value.split(delimiter_pattern, 3);
          info[key] = [parseFloat(ss[0]), parseFloat(ss[1]), parseFloat(ss[2])];
        } else {
          info[key] = value;
        }
      }
    }

    const materialCreator = new THREE.MTLLoader.MaterialCreator(
      this.texturePath || this.path,
      this.materialOptions
    );
    materialCreator.setCrossOrigin(this.crossOrigin);
    materialCreator.setManager(this.manager);
    materialCreator.setMaterials(materialsInfo);
    return materialCreator;
  },
};

/**
 * Create a new THREE-MTLLoader.MaterialCreator
 * @param baseUrl - Url relative to which textures are loaded
 * @param options - Set of options on how to construct the materials
 *                  side: Which side to apply the material
 *                        THREE.FrontSide (default), THREE.BackSide,
 * THREE.DoubleSide wrap: What type of wrapping to apply for textures
 *                        THREE.RepeatWrapping (default),
 * THREE.ClampToEdgeWrapping, THREE.MirroredRepeatWrapping normalizeRGB: RGBs
 * need to be normalized to 0-1 from 0-255 Default: false, assumed to be already
 * normalized ignoreZeroRGBs: Ignore values of RGBs (Ka,Kd,Ks) that are all 0's
 *                                  Default: false
 * @constructor
 */

THREE.MTLLoader.MaterialCreator = function (baseUrl, options) {
  this.baseUrl = baseUrl || "";
  this.options = options;
  this.materialsInfo = {};
  this.materials = {};
  this.materialsArray = [];
  this.nameLookup = {};

  this.side =
    this.options && this.options.side ? this.options.side : THREE.FrontSide;
  this.wrap =
    this.options && this.options.wrap
      ? this.options.wrap
      : THREE.RepeatWrapping;
};

THREE.MTLLoader.MaterialCreator.prototype = {
  constructor: THREE.MTLLoader.MaterialCreator,

  setCrossOrigin: function (value) {
    this.crossOrigin = value;
  },

  setManager: function (value) {
    this.manager = value;
  },

  setMaterials: function (materialsInfo) {
    this.materialsInfo = this.convert(materialsInfo);
    this.materials = {};
    this.materialsArray = [];
    this.nameLookup = {};
  },

  convert: function (materialsInfo) {
    if (!this.options) return materialsInfo;

    const converted = {};

    for (const mn in materialsInfo) {
      // Convert materials info into normalized form based on options

      const mat = materialsInfo[mn];

      const covmat = {};

      converted[mn] = covmat;

      for (const prop in mat) {
        let save = true;
        let value = mat[prop];
        const lprop = prop.toLowerCase();

        switch (lprop) {
          case "kd":
          case "ka":
          case "ks":
            // Diffuse color (color under white light) using RGB values

            if (this.options && this.options.normalizeRGB) {
              value = [value[0] / 255, value[1] / 255, value[2] / 255];
            }

            if (this.options && this.options.ignoreZeroRGBs) {
              if (value[0] === 0 && value[1] === 0 && value[2] === 0) {
                // ignore

                save = false;
              }
            }

            break;

          default:
            break;
        }

        if (save) {
          covmat[lprop] = value;
        }
      }
    }

    return converted;
  },

  preload: function () {
    for (const mn in this.materialsInfo) {
      this.create(mn);
    }
  },

  getIndex: function (materialName) {
    return this.nameLookup[materialName];
  },

  getAsArray: function () {
    let index = 0;

    for (const mn in this.materialsInfo) {
      this.materialsArray[index] = this.create(mn);
      this.nameLookup[mn] = index;
      index++;
    }

    return this.materialsArray;
  },

  create: function (materialName) {
    if (this.materials[materialName] === undefined) {
      this.createMaterial_(materialName);
    }

    return this.materials[materialName];
  },

  createMaterial_: function (materialName) {
    // Create material

    const scope = this;
    const mat = this.materialsInfo[materialName];
    const params = {
      name: materialName,
      side: this.side,
    };

    function resolveURL(baseUrl, url) {
      if (typeof url !== "string" || url === "") {
        return "";
      }

      // Absolute URL
      if (/^https?:\/\//i.test(url)) return url;

      return baseUrl + url;
    }

    function setMapForType(mapType, value) {
      if (params[mapType]) return; // Keep the first encountered texture

      const texParams = scope.getTextureParams(value, params);
      const map = scope.loadTexture(resolveURL(scope.baseUrl, texParams.url));

      map.repeat.copy(texParams.scale);
      map.offset.copy(texParams.offset);

      map.wrapS = scope.wrap;
      map.wrapT = scope.wrap;

      params[mapType] = map;
    }

    for (const prop in mat) {
      const value = mat[prop];
      var n;

      if (value === "") continue;

      switch (prop.toLowerCase()) {
        // Ns is material specular exponent

        case "kd":
          // Diffuse color (color under white light) using RGB values

          params.color = new THREE.Color().fromArray(value);

          break;

        case "ks":
          // Specular color (color when light is reflected from shiny surface)
          // using RGB values
          params.specular = new THREE.Color().fromArray(value);

          break;

        case "map_kd":
          // Diffuse texture map

          setMapForType("map", value);

          break;

        case "map_ks":
          // Specular map

          setMapForType("specularMap", value);

          break;

        case "map_bump":
        case "bump":
          // Bump texture map

          setMapForType("bumpMap", value);

          break;

        case "ns":
          // The specular exponent (defines the focus of the specular highlight)
          // A high exponent results in a tight, concentrated highlight. Ns values
          // normally range from 0 to 1000.

          params.shininess = parseFloat(value);

          break;

        case "d":
          n = parseFloat(value);

          if (n < 1) {
            params.opacity = n;
            params.transparent = true;
          }

          break;

        case "tr":
          n = parseFloat(value);

          if (n > 0) {
            params.opacity = 1 - n;
            params.transparent = true;
          }

          break;

        default:
          break;
      }
    }

    this.materials[materialName] = new THREE.MeshPhongMaterial(params);
    return this.materials[materialName];
  },

  getTextureParams: function (value, matParams) {
    const texParams = {
      scale: new THREE.Vector2(1, 1),
      offset: new THREE.Vector2(0, 0),
    };

    const items = value.split(/\s+/);
    let pos;

    pos = items.indexOf("-bm");

    if (pos >= 0) {
      matParams.bumpScale = parseFloat(items[pos + 1]);
      items.splice(pos, 2);
    }

    pos = items.indexOf("-s");

    if (pos >= 0) {
      texParams.scale.set(
        parseFloat(items[pos + 1]),
        parseFloat(items[pos + 2])
      );
      items.splice(pos, 4); // we expect 3 parameters here!
    }

    pos = items.indexOf("-o");

    if (pos >= 0) {
      texParams.offset.set(
        parseFloat(items[pos + 1]),
        parseFloat(items[pos + 2])
      );
      items.splice(pos, 4); // we expect 3 parameters here!
    }

    texParams.url = items.join(" ").trim();
    return texParams;
  },

  loadTexture: function (url, mapping, onLoad, onProgress, onError) {
    let texture;
    let loader = THREE.Loader.Handlers.get(url);
    const manager =
      this.manager !== undefined ? this.manager : THREE.DefaultLoadingManager;

    if (loader === null) {
      loader = new THREE.TextureLoader(manager);
    }

    if (loader.setCrossOrigin) loader.setCrossOrigin(this.crossOrigin);
    texture = loader.load(url, onLoad, onProgress, onError);

    if (mapping !== undefined) texture.mapping = mapping;

    return texture;
  },
};
