//
// Created by mason on 4/11/20.
//

#ifndef LIBTRT_TRTUTILS_HPP
#define LIBTRT_TRTUTILS_HPP

template<typename T>
struct TRTDeleter {
    void operator()(T* ptr) {
        ptr->destroy();
    }
};

#endif //LIBTRT_TRTUTILS_HPP
